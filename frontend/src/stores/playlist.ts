import { defineStore } from 'pinia';

export interface Track {
  id: string;
  title: string;
  artist: string;
  durationSeconds: number;
  thumbnailUrl: string;
  originalUrl: string;
}

const TOTAL_CD_CAPACITY_SECONDS = 80 * 60; // 4800s (80 mins)

export const usePlaylistStore = defineStore('playlist', {
  state: () => ({
    playlistName: '',
    tracks: [] as Track[],
    isLoadingTrack: false,
    errorMessage: '',
  }),

  getters: {
    // Total duration in seconds
    totalDurationSeconds(state): number {
      return state.tracks.reduce((acc, track) => acc + (track.durationSeconds || 0), 0);
    },

    // Formatted string (e.g., "12m 34s")
    totalDurationFormatted(): string {
      const mins = Math.floor(this.totalDurationSeconds / 60);
      const secs = this.totalDurationSeconds % 60;
      return `${mins}m ${secs.toString().padStart(2, '0')}s`;
    },

    // Percentage for capacity progress bar
    capacityPercentage(): number {
      return Math.min((this.totalDurationSeconds / TOTAL_CD_CAPACITY_SECONDS) * 100, 100);
    },

    // Boolean check if total length exceeds 80 mins
    isOverCdLimit(): boolean {
      return this.totalDurationSeconds > TOTAL_CD_CAPACITY_SECONDS;
    },
  },

  actions: {
    async addTrackFromUrl(url: string) {
      this.isLoadingTrack = true;
      this.errorMessage = '';

      try {
        const response = await fetch('http://localhost:5000/api/track/info', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ url }),
        });

        const data = await response.json();

        if (!response.ok) {
          throw new Error(data.error || 'Failed to fetch song details.');
        }

        if (data.track) {
          this.tracks.push(data.track);
        }
      } catch (err: any) {
        this.errorMessage = err.message || 'Something went wrong while adding the track.';
      } finally {
        this.isLoadingTrack = false;
      }
    },

    removeTrack(trackId: string) {
      this.tracks = this.tracks.filter((track) => track.id !== trackId);
    },
  },
});