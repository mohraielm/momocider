import express from 'express';
import cors from 'cors';
import ytdlp from 'yt-dlp-exec';
import ffmpegPath from 'ffmpeg-static';
import path from 'path';
import fs from 'fs';
import { exec } from 'child_process';
import { promisify } from 'util';
import archiver from 'archiver';

const execAsync = promisify(exec);
const app = express();
const PORT = process.env.PORT || 5000;
const ytDlpBaseOptions = {
  noWarnings: true,
  noCallHome: true,
  preferFreeFormats: true,
  noProgress: true,
  update: false,
  ffmpegLocation: ffmpegPath,
};

app.use(cors());
app.use(express.json());

// 📡 1. Metadata Fetching Endpoint
app.post('/api/track/info', async (req, res) => {
  const { url } = req.body;
  if (!url) {
    return res.status(400).json({ error: 'Please provide a valid track URL.' });
  }

  try {
    const metadata = await ytdlp(url, {
      ...ytDlpBaseOptions,
      dumpSingleJson: true,
    });

    const trackInfo = {
      id: metadata.id,
      title: metadata.title || metadata.track || 'Unknown Title',
      artist: metadata.artist || metadata.uploader || metadata.channel || 'Unknown Artist',
      durationSeconds: Math.round(metadata.duration || 0),
      thumbnailUrl: metadata.thumbnail || '',
      originalUrl: url,
    };

    return res.json({ success: true, track: trackInfo });
  } catch (error) {
    console.error('Error fetching track metadata:', error);
    return res.status(500).json({ error: 'Failed to extract metadata.' });
  }
});

// 💿 2. MP3 Download & ZIP Packaging Endpoint
app.post('/api/prepare-burn', async (req, res) => {
  const { tracks, playlistName } = req.body;
  console.log(`💿 Burn prep request received for "${playlistName}" (${tracks?.length} tracks)`);

  if (!tracks || !Array.isArray(tracks) || tracks.length === 0) {
    return res.status(400).json({ error: 'No tracks provided for burn preparation.' });
  }

  const sanitizedTitle = (playlistName || 'momocider_mixtape').replace(/[^a-z0-9]/gi, '_').toLowerCase();
  const tempDir = path.join(process.cwd(), 'temp', `burn_${sanitizedTitle}_${Date.now()}`);

  try {
    await fs.promises.mkdir(tempDir, { recursive: true });
    const preparedTracks = [];

    for (let i = 0; i < tracks.length; i++) {
      const track = tracks[i];
      const targetUrl = track.url || track.originalUrl;
      if (!targetUrl) continue;

      const trackNum = (i + 1).toString().padStart(2, '0');
      const outputTemplate = path.join(tempDir, `${trackNum} - %(title)s.%(ext)s`);
      console.log(`⬇️ [${i + 1}/${tracks.length}] Preparing burn MP3: ${targetUrl}`);

      await ytdlp(targetUrl, {
        ...ytDlpBaseOptions,
        extractAudio: true,
        audioFormat: 'mp3',
        audioQuality: 0,
        output: outputTemplate,
      });

      const files = await fs.promises.readdir(tempDir);
      const mp3File = files
        .filter(file => file.toLowerCase().endsWith('.mp3'))
        .sort()
        .slice(-1)[0];

      if (mp3File) {
        preparedTracks.push({
          id: track.id || `${Date.now()}-${i}`,
          title: track.title || `Track ${i + 1}`,
          path: path.join(tempDir, mp3File),
        });
      }
    }

    if (preparedTracks.length === 0) {
      return res.status(500).json({
        error: 'No MP3 files were produced from the selected tracks. The conversion step failed.',
      });
    }

    return res.json({ success: true, tracks: preparedTracks });
  } catch (error) {
    const details = error?.stderr || error?.stdout || error?.message || String(error);
    console.error('❌ Burn preparation failed:', details);
    return res.status(500).json({
      error: 'Failed to prepare MP3 tracks for burning.',
      details,
    });
  }
});

app.post('/api/export-zip', async (req, res) => {
  const { tracks, playlistName } = req.body;
  console.log(`📦 Export request received for "${playlistName}" (${tracks?.length} tracks)`);

  if (!tracks || !Array.isArray(tracks) || tracks.length === 0) {
    return res.status(400).json({ error: 'No tracks provided for export.' });
  }

  const sanitizedTitle = (playlistName || 'momocider_mixtape').replace(/[^a-z0-9]/gi, '_').toLowerCase();
  const tempDir = path.join(process.cwd(), 'temp', `${sanitizedTitle}_${Date.now()}`);

  try {
    await fs.promises.mkdir(tempDir, { recursive: true });

    for (let i = 0; i < tracks.length; i++) {
      const track = tracks[i];
      const targetUrl = track.url || track.originalUrl;
      
      if (!targetUrl) continue;

      const trackNum = (i + 1).toString().padStart(2, '0');
      const outputTemplate = path.join(tempDir, `${trackNum} - %(title)s.%(ext)s`);

      console.log(`⬇️ [${i + 1}/${tracks.length}] Downloading: ${targetUrl}`);

      await ytdlp(targetUrl, {
        ...ytDlpBaseOptions,
        extractAudio: true,
        audioFormat: 'mp3',
        audioQuality: 0,
        output: outputTemplate,
      });
    }

    console.log('⚡ All downloads complete. Creating ZIP archive...');

    res.setHeader('Content-Type', 'application/zip');
    res.setHeader('Content-Disposition', `attachment; filename="${sanitizedTitle}_cd_pack.zip"`);

    const archive = archiver('zip', { 
        zlib: { level: 9 } 
    });

    archive.pipe(res);

    const files = await fs.promises.readdir(tempDir);
    for (const file of files) {
      if (file.endsWith('.mp3')) {
        archive.file(path.join(tempDir, file), { name: file });
      }
    }

    await archive.finalize();
    console.log('✅ ZIP sent successfully to browser!');

    res.on('finish', async () => {
      await fs.promises.rm(tempDir, { recursive: true, force: true }).catch(() => {});
    });

  } catch (error) {
    console.error('❌ Export failed:', error);
    if (fs.existsSync(tempDir)) {
      await fs.promises.rm(tempDir, { recursive: true, force: true }).catch(() => {});
    }
    return res.status(500).json({ error: 'Failed to package MP3s.' });
  }
});

// --- 💿 CD Burn Endpoint ---
app.post('/api/burn-cd', async (req, res) => {
    const { playlistName, tracks } = req.body;

    if (!tracks || tracks.length === 0) {
        return res.status(400).json({ success: false, error: 'No tracks provided to burn.' });
    }

    console.log(`💿 CD Burn request received for "${playlistName}" (${tracks.length} tracks)`);

    try {
        return res.status(501).json({
            success: false,
            error: 'Real disc burning is handled by the Windows Tauri IMAPI layer, not the Node backend. The backend only handles track metadata and zip export.'
        });
    } catch (err) {
        console.error('❌ Burn process failed:', err);
        return res.status(500).json({ success: false, error: err.message || 'Failed to burn CD.' });
    }
});

// 🚀 Start Express Server
const server = app.listen(PORT, () => {
    console.log(`🍑 Momocider backend running on http://localhost:${PORT}`);
});

server.timeout = 600000;