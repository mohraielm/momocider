<script setup lang="ts">
    import { ref } from 'vue';
    import JewelCaseIntro from './components/JewelCaseIntro.vue';
    import CdCoverDesigner from './components/CdCoverDesigner.vue';
    import CdBurnModal from './components/CdBurnModal.vue'; 
    import { usePlaylistStore } from './stores/playlist';

    const showIntro = ref(true);
    const inputUrl = ref('');
    const store = usePlaylistStore();

    function handleIntroComplete() {
        showIntro.value = false;
    }

    async function handleAddSong() {
        if (!inputUrl.value.trim()) return;
        await store.addTrackFromUrl(inputUrl.value);
        inputUrl.value = '';
    }
</script>

<template>
    <div class="app-root">
        <JewelCaseIntro v-if="showIntro" @animation-complete="handleIntroComplete" />

        <main v-else class="momocider-station">
            <header class="station-header">
                <h1>🍑 MomoCider</h1>
                <p class="subtitle">• Digital CD Mixtape Station •</p>
            </header>

            <section class="card">
                <!-- Mixtape Title -->
                <div class="playlist-title-wrapper">
                    <label for="playlistName">Mixtape Name:</label>
                    <input id="playlistName"
                           v-model="store.playlistName"
                           type="text"
                           class="peach-input title-input"
                           placeholder="Name your mixtape..." />
                </div>

                <!-- 80-Min Capacity Bar -->
                <div class="capacity-container">
                    <div class="capacity-header">
                        <span>💿 CD Capacity Meter</span>
                        <span>{{ store.totalDurationFormatted }} / 80m 00s</span>
                    </div>
                    <div class="progress-bar-bg">
                        <div class="progress-bar-fill"
                             :class="{ 'over-limit': store.isOverCdLimit }"
                             :style="{ width: `${store.capacityPercentage}%` }"></div>
                    </div>
                    <p v-if="store.isOverCdLimit" class="warning-text">
                        ⚠️ Playlist exceeds physical 80-minute CD limit!
                    </p>
                </div>

                <!-- Link Input Form -->
                <form @submit.prevent="handleAddSong" class="add-track-form">
                    <input v-model="inputUrl"
                           type="url"
                           class="peach-input"
                           placeholder="Paste YouTube song link..."
                           :disabled="store.isLoadingTrack"
                           required />
                    <button type="submit" class="peach-button" :disabled="store.isLoadingTrack">
                        {{ store.isLoadingTrack ? 'Processing...' : '+ Add Track' }}
                    </button>
                </form>

                <p v-if="store.errorMessage" class="error-text">{{ store.errorMessage }}</p>

                <!-- Tracklist & Export Controls -->
                <div class="track-list">
                    <div class="tracklist-header">
                        <h3>Tracklist ({{ store.tracks.length }})</h3>
                        <div class="export-actions" v-if="store.tracks.length > 0">
                            <button class="export-btn mp3-btn" @click="store.exportMp3Zip" :disabled="store.isLoadingTrack" title="Download zipped MP3 files ready for burning">
                                {{ store.isLoadingTrack ? '⏳ Zipping MP3s...' : '💿 Export ZIP' }}
                            </button>
                            
                            <!-- 👈 2. Drop the Burn Modal right here! -->
                            <CdBurnModal />

                            <button class="export-btn" @click="store.exportM3U">💾 .M3U</button>
                            <button class="export-btn" @click="store.exportJSON">📄 JSON</button>
                        </div>
                    </div>

                    <div v-if="store.tracks.length === 0" class="empty-state">
                        No tracks added yet. Paste a YouTube link above to begin!
                    </div>

                    <div v-for="(track, index) in store.tracks" :key="track.id" class="track-item">
                        <span class="track-number">{{ (index + 1).toString().padStart(2, '0') }}</span>
                        <img :src="track.thumbnailUrl" alt="cover" class="track-thumb" />
                        <div class="track-info">
                            <span class="track-title">{{ track.title }}</span>
                            <span class="track-artist">{{ track.artist }}</span>
                        </div>
                        <button @click="store.removeTrack(track.id)" class="remove-btn" title="Remove track">✕</button>
                    </div>
                </div>

                <!-- Customizer Modal Button -->
                <CdCoverDesigner />
            </section>
        </main>
    </div>
</template>

<style scoped>
    /* Scoped Styles for Main Station */
    .app-root {
        min-height: 100vh;
        background: radial-gradient(circle, #424f44 0%, #2e362f 100%);
        color: #e5ece6;
        font-family: 'Courier New', Courier, monospace;
        padding-bottom: 2rem;
    }

    .momocider-station {
        max-width: 680px;
        margin: 0 auto;
        padding: 2.5rem 1rem;
    }

    .station-header {
        text-align: center;
        margin-bottom: 2rem;
    }

    .station-header h1 {
        font-size: 2.8rem;
        margin: 0;
        color: #e8c5b8;
    }

    .subtitle {
        margin-top: 0.3rem;
        color: #a3b8a7;
        font-size: 0.9rem;
    }

    .card {
        background: #2b332d;
        border: 3px solid #1e2420;
        border-radius: 16px;
        padding: 1.8rem;
        box-shadow: 8px 8px 0px #1a211c;
    }

    .playlist-title-wrapper {
        margin-bottom: 1.5rem;
    }

    .playlist-title-wrapper label {
        display: block;
        font-weight: bold;
        font-size: 0.85rem;
        margin-bottom: 0.4rem;
        color: #b3c7b7;
    }

    .peach-input {
        width: 100%;
        box-sizing: border-box;
        padding: 0.75rem 1rem;
        border: 2px solid #1e2420;
        border-radius: 8px;
        background: #38433a;
        font-family: inherit;
        font-size: 0.95rem;
        color: #ffffff;
        outline: none;
    }

    .peach-input:focus {
        border-color: #a3c9a8;
    }

    .title-input {
        font-weight: bold;
        font-size: 1.1rem;
    }

    .capacity-container {
        background: #323b34;
        border: 2px dashed #526355;
        border-radius: 10px;
        padding: 1rem;
        margin-bottom: 1.5rem;
    }

    .capacity-header {
        display: flex;
        justify-content: space-between;
        font-size: 0.85rem;
        font-weight: bold;
        margin-bottom: 0.5rem;
        color: #c0d1c3;
    }

    .progress-bar-bg {
        width: 100%;
        height: 16px;
        background: #232a24;
        border-radius: 8px;
        overflow: hidden;
        border: 1px solid #1e2420;
    }

    .progress-bar-fill {
        height: 100%;
        background: linear-gradient(90deg, #718073, #a3c9a8);
        transition: width 0.4s ease;
    }

    .progress-bar-fill.over-limit {
        background: linear-gradient(90deg, #e87c7c, #ff4d4d);
    }

    .warning-text {
        color: #ff7b7b;
        font-size: 0.8rem;
        margin: 0.5rem 0 0 0;
        font-weight: bold;
    }

    .error-text {
        color: #ff7b7b;
        font-size: 0.85rem;
        margin-bottom: 1rem;
    }

    .empty-state {
        text-align: center;
        padding: 2rem 1rem;
        color: #8fa393;
        font-size: 0.85rem;
        border: 1px dashed #38433a;
        border-radius: 8px;
    }

    .add-track-form {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .peach-button {
        background: #7f9a84;
        border: 2px solid #1e2420;
        border-radius: 8px;
        padding: 0.75rem 1.2rem;
        font-family: inherit;
        font-weight: bold;
        color: #ffffff;
        cursor: pointer;
        box-shadow: 3px 3px 0px #1e2420;
    }

    .peach-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .track-list {
        margin-top: 1.5rem;
    }

    .tracklist-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.8rem;
    }

    .tracklist-header h3 {
        margin: 0;
        color: #c0d1c3;
    }

    .export-actions {
        display: flex;
        gap: 0.4rem;
        align-items: center;
    }

    .export-btn {
        background: #38433a;
        border: 1px solid #526355;
        border-radius: 6px;
        padding: 0.35rem 0.6rem;
        color: #e5ece6;
        font-size: 0.75rem;
        cursor: pointer;
    }

    .export-btn:disabled {
        opacity: 0.5;
        cursor: wait;
    }

    .mp3-btn {
        background: #7f9a84;
        color: #fff;
        font-weight: bold;
    }

    .track-item {
        display: flex;
        align-items: center;
        gap: 0.8rem;
        background: #38433a;
        border: 2px solid #1e2420;
        border-radius: 8px;
        padding: 0.6rem;
        margin-bottom: 0.6rem;
    }

    .track-number {
        font-weight: bold;
        color: #a3c9a8;
    }

    .track-thumb {
        width: 44px;
        height: 44px;
        object-fit: cover;
        border-radius: 4px;
    }

    .track-info {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    .track-title {
        font-weight: bold;
        font-size: 0.85rem;
        color: #ffffff;
    }

    .track-artist {
        font-size: 0.75rem;
        color: #a3b8a7;
    }

    .remove-btn {
        background: transparent;
        border: none;
        color: #8fa393;
        font-size: 1.1rem;
        cursor: pointer;
    }
</style>