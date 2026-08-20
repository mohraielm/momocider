import { defineStore } from 'pinia';

export interface Track {
    id: string;
    youtubeUrl: string;
    title: string;
    artist: string;
    durationSeconds: number;
    thumbnailUrl: string;
}

export const usePlaylistStore = defineStore('playlist', {
    state: () => ({
        playlistName: 'My Peach Mixtape',
        tracks: [] as Track[],
        isLoadingTrack: false,
        errorMessage: null as string | null
    }),
    getters: {
        totalDurationSeconds: (state) =>
            state.tracks.reduce((acc, track) => acc + track.durationSeconds, 0),

        totalDurationFormatted(): string {
            const mins = Math.floor(this.totalDurationSeconds / 60);
            const secs = this.totalDurationSeconds % 60;
            return `${mins}m ${secs < 10 ? '0' : ''}${secs}s`;
        },

        // 80 minutes = 4800 seconds max on standard Audio CD
        capacityPercentage(): number {
            return Math.min(100, Math.round((this.totalDurationSeconds / 4800) * 100));
        },

        isOverCdLimit(): boolean {
            return this.totalDurationSeconds > 4800;
        }
    },
    actions: {
        async addTrackFromUrl(url: string) {
            this.isLoadingTrack = true;
            this.errorMessage = null;

            try {
                const response = await fetch('http://localhost:5000/api/tracks/info', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ url })
                });

                if (!response.ok) {
                    throw new Error('Could not fetch song info. Check the YouTube link.');
                }

                const data = await response.json();
                this.tracks.push({
                    id: crypto.randomUUID(),
                    youtubeUrl: url,
                    title: data.title,
                    artist: data.artist,
                    durationSeconds: data.durationSeconds,
                    thumbnailUrl: data.thumbnailUrl
                });
            } catch (err: any) {
                this.errorMessage = err.message || 'Error adding track.';
            } finally {
                this.isLoadingTrack = false;
            }
        },

        removeTrack(id: string) {
            this.tracks = this.tracks.filter(t => t.id !== id);
        }
    }
});