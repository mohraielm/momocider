import { defineStore } from 'pinia';

export interface Track {
    id: string;
    title: string;
    artist: string;
    durationSeconds: number;
    thumbnailUrl: string;
    url: string;
}

export const usePlaylistStore = defineStore('playlist', {
    state: () => ({
        playlistName: 'Momocider Vol. 1',
        subtitle: 'Digital Mixtape Station',
        coverTheme: 'forest' as 'forest' | 'peach' | 'lavender' | 'midnight',
        coverIcon: '🌸',
        tracks: [] as Track[],
        isLoadingTrack: false,
        errorMessage: '',
        
        // --- Playback State ---
        currentTrackIndex: 0,
        isPlaying: false,
        currentTimeSeconds: 0,
    }),
    getters: {
        currentTrack(state): Track | null {
            if (state.tracks.length === 0) return null;
            return state.tracks[state.currentTrackIndex] || null;
        },
        totalDurationSeconds(state): number {
            return state.tracks.reduce((acc, t) => acc + t.durationSeconds, 0);
        },
        totalDurationFormatted(): string {
            const mins = Math.floor(this.totalDurationSeconds / 60);
            const secs = this.totalDurationSeconds % 60;
            return `${mins}m ${secs.toString().padStart(2, '0')}s`;
        },
        capacityPercentage(): number {
            const MAX_CD_SECONDS = 80 * 60;
            return Math.min(100, (this.totalDurationSeconds / MAX_CD_SECONDS) * 100);
        },
        isOverCdLimit(): boolean {
            return this.totalDurationSeconds > 80 * 60;
        }
    },
    actions: {
        async addTrackFromUrl(url: string) {
            this.isLoadingTrack = true;
            this.errorMessage = '';
            try {
                const response = await fetch('http://localhost:5000/api/track/info', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ url })
                });

                const data = await response.json();
                if (!response.ok || !data.success) {
                    throw new Error(data.error || 'Failed to fetch track details.');
                }

                const newTrack: Track = {
                    id: data.track.id || Date.now().toString(),
                    title: data.track.title,
                    artist: data.track.artist,
                    durationSeconds: data.track.durationSeconds,
                    thumbnailUrl: data.track.thumbnailUrl,
                    url: data.track.originalUrl
                };

                this.tracks.push(newTrack);
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to fetch YouTube track details.';
            } finally {
                this.isLoadingTrack = false;
            }
        },
        removeTrack(id: string) {
            this.tracks = this.tracks.filter(t => t.id !== id);
            if (this.currentTrackIndex >= this.tracks.length) {
                this.currentTrackIndex = Math.max(0, this.tracks.length - 1);
            }
        },
        
        // --- Playback Controls ---
        togglePlay() {
            if (!this.currentTrack) return;
            this.isPlaying = !this.isPlaying;
        },
        playTrack(index: number) {
            if (index >= 0 && index < this.tracks.length) {
                this.currentTrackIndex = index;
                this.isPlaying = true;
                this.currentTimeSeconds = 0;
            }
        },
        nextTrack() {
            if (this.tracks.length === 0) return;
            this.currentTrackIndex = (this.currentTrackIndex + 1) % this.tracks.length;
            this.isPlaying = true;
            this.currentTimeSeconds = 0;
        },
        previousTrack() {
            if (this.tracks.length === 0) return;
            this.currentTrackIndex = (this.currentTrackIndex - 1 + this.tracks.length) % this.tracks.length;
            this.isPlaying = true;
            this.currentTimeSeconds = 0;
        },

        // --- Export & Burn Handlers ---
        async exportMp3Zip() {
            if (this.tracks.length === 0) return;

            this.isLoadingTrack = true;
            this.errorMessage = '';

            try {
                const response = await fetch('http://localhost:5000/api/export-zip', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        playlistName: this.playlistName,
                        tracks: this.tracks
                    })
                });

                if (!response.ok) throw new Error('Failed to generate ZIP on backend');

                const blob = await response.blob();
                const url = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = `${(this.playlistName || 'mixtape').toLowerCase().replace(/\s+/g, '-')}-cd-pack.zip`;
                document.body.appendChild(a);
                a.click();
                a.remove();
                window.URL.revokeObjectURL(url);
            } catch (err: any) {
                this.errorMessage = 'Failed to export MP3 zip package. Check backend server on port 5000.';
            } finally {
                this.isLoadingTrack = false;
            }
        },
        exportM3U() {
            let content = '#EXTM3U\n';
            this.tracks.forEach(t => {
                content += `#EXTINF:${t.durationSeconds},${t.artist} - ${t.title}\n${t.url}\n`;
            });
            const blob = new Blob([content], { type: 'audio/x-mpegurl' });
            const link = document.createElement('a');
            link.href = URL.createObjectURL(blob);
            link.download = `${this.playlistName.toLowerCase().replace(/\s+/g, '-')}.m3u`;
            link.click();
        },
        exportJSON() {
            const data = {
                name: this.playlistName,
                subtitle: this.subtitle,
                theme: this.coverTheme,
                tracks: this.tracks
            };
            const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
            const link = document.createElement('a');
            link.href = URL.createObjectURL(blob);
            link.download = `${this.playlistName.toLowerCase().replace(/\s+/g, '-')}.json`;
            link.click();
        },
        async burnCdToDrive() {
            if (this.tracks.length === 0) return;

            this.isLoadingTrack = true;
            this.errorMessage = '';

            try {
                const response = await fetch('http://localhost:5000/api/burn-cd', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        playlistName: this.playlistName,
                        tracks: this.tracks
                    })
                });

                const data = await response.json();
                if (!response.ok) throw new Error(data.error || 'Burn process failed');

                alert('💿 Success! Your audio CD has been burned.');
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to burn CD. Check local backend.';
                alert(`❌ ${this.errorMessage}`);
            } finally {
                this.isLoadingTrack = false;
            }
        }
    }
});