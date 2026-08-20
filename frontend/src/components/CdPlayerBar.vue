<script setup lang="ts">
  import { usePlaylistStore } from "../stores/playlist";

  const store = usePlaylistStore();

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }
</script>

<template>
  <div v-if="store.tracks.length > 0" class="player-bar-wrapper">
    <div class="player-bar">
      <!-- Left: Animated Mini Disc & Song Info -->
      <div class="track-details">
        <div class="mini-disc" :class="{ 'is-spinning': store.isPlaying }">
          <img
            :src="store.currentTrack?.thumbnailUrl"
            alt="art"
            class="mini-art"
          />
          <div class="mini-hub"></div>
        </div>
        <div class="meta">
          <span class="title">{{
            store.currentTrack?.title || "No Track Selected"
          }}</span>
          <span class="artist">{{
            store.currentTrack?.artist || "Unknown"
          }}</span>
        </div>
      </div>

      <!-- Middle: Transport Controls & Scrubber -->
      <div class="player-center">
        <div class="controls">
          <button
            class="ctrl-btn"
            @click="store.previousTrack"
            title="Previous Track"
          >
            ⏮
          </button>
          <button
            class="ctrl-btn play-btn"
            @click="store.togglePlay"
            title="Play / Pause"
          >
            {{ store.isPlaying ? "⏸" : "▶" }}
          </button>
          <button class="ctrl-btn" @click="store.nextTrack" title="Next Track">
            ⏭
          </button>
        </div>

        <div class="scrubber-row">
          <span class="time-stamp">{{
            formatTime(store.currentTimeSeconds)
          }}</span>
          <div class="track-progress-bg">
            <div
              class="track-progress-fill"
              :style="{ width: store.currentTrack ? `${(store.currentTimeSeconds / store.currentTrack.durationSeconds) * 100}%` : '0%' }"
            ></div>
          </div>
          <span class="time-stamp">{{
            formatTime(store.currentTrack?.durationSeconds || 0)
          }}</span>
        </div>
      </div>

      <!-- Right: Display Track Number -->
      <div class="track-counter">
        <span
          >TRK {{ (store.currentTrackIndex + 1).toString().padStart(2, "0") }} /
          {{ store.tracks.length.toString().padStart(2, "0") }}</span
        >
      </div>
    </div>
  </div>
</template>

<style scoped>
  .player-bar-wrapper {
    position: fixed;
    bottom: 1.5rem;
    left: 50%;
    transform: translateX(-50%);
    width: calc(100% - 2rem);
    max-width: 680px;
    z-index: 50;
    animation: slideUp 0.4s ease-out;
  }

  .player-bar {
    background: #232a24;
    border: 2px solid #526355;
    border-radius: 16px;
    padding: 0.75rem 1.2rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
  }

  /* Mini Disc Visual */
  .track-details {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    flex: 1;
    min-width: 0;
  }

  .mini-disc {
    position: relative;
    width: 42px;
    height: 42px;
    border-radius: 50%;
    background: conic-gradient(from 0deg, #d1e0d3, #f2dede, #b8cbd0, #d1e0d3);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    flex-shrink: 0;
  }

  .mini-disc.is-spinning {
    animation: spin 3s linear infinite;
  }

  .mini-art {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    object-fit: cover;
  }

  .mini-hub {
    position: absolute;
    width: 8px;
    height: 8px;
    background: #232a24;
    border: 1px solid #7f9a84;
    border-radius: 50%;
  }

  .meta {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .meta .title {
    font-size: 0.85rem;
    font-weight: bold;
    color: #e5ece6;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta .artist {
    font-size: 0.7rem;
    color: #8fa393;
  }

  /* Player Controls */
  .player-center {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.3rem;
    flex: 1.2;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .ctrl-btn {
    background: #323b34;
    border: 1px solid #526355;
    color: #e5ece6;
    border-radius: 50%;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 0.8rem;
    transition: background 0.15s ease;
  }

  .ctrl-btn:hover {
    background: #526355;
  }

  .play-btn {
    width: 38px;
    height: 38px;
    background: #7f9a84;
    border-color: #a3c9a8;
    font-size: 0.95rem;
  }

  .play-btn:hover {
    background: #92ac97;
  }

  .scrubber-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
  }

  .time-stamp {
    font-size: 0.65rem;
    color: #8fa393;
  }

  .track-progress-bg {
    flex: 1;
    height: 5px;
    background: #181e19;
    border-radius: 3px;
    overflow: hidden;
  }

  .track-progress-fill {
    height: 100%;
    background: #a3c9a8;
    transition: width 0.3s linear;
  }

  .track-counter {
    font-size: 0.75rem;
    font-weight: bold;
    color: #a3c9a8;
    letter-spacing: 1px;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes slideUp {
    from {
      transform: translate(-50%, 100%);
      opacity: 0;
    }
    to {
      transform: translate(-50%, 0);
      opacity: 1;
    }
  }
</style>
