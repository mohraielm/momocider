import express from 'express';
import cors from 'cors';
import ytdlp from 'yt-dlp-exec';

const app = express();
const PORT = process.env.PORT || 5000;

app.use(cors());
app.use(express.json());

// 📡 Step 1: The Gathering - Parse Track Metadata
app.post('/api/track/info', async (req, res) => {
  const { url } = req.body;

  if (!url) {
    return res.status(400).json({ error: 'Please provide a valid track URL.' });
  }

  try {
    const metadata = await ytdlp(url, {
      dumpSingleJson: true,
      noWarnings: true,
      noCallHome: true,
      preferFreeFormats: true,
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
    return res.status(500).json({ 
      error: 'Failed to extract metadata. Check URL validity or yt-dlp installation.' 
    });
  }
});

app.listen(PORT, () => {
  console.log(`🍑 Momocider backend running on http://localhost:${PORT}`);
});