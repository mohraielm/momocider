<script setup lang="ts">
    import { ref } from 'vue';
    import JewelCaseIntro from './components/JewelCaseIntro.vue';
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
        <JewelCaseIntro v-if="showIntro"
                        @animation-complete="handleIntroComplete" />

        <main v-else class="momocider-station">
            <!-- Header -->
            <header class="station-header">
                <h1>🍑 Momocider</h1>
                <p class="subtitle">桃サイダー • Digital CD Mixtape Station</p>
            </header>

            <!-- Main CD Station Card -->
            <section class="card">
                <!-- Playlist Title Input -->
                <div class="playlist-title-wrapper">
                    <label for="playlistName">Mixtape Name:</label>
                    <input id="playlistName"
                           v-model="store.playlistName"
                           type="text"
                           class="peach-input title-input"
                           placeholder="Name your mixtape..." />
                </div>

                <!-- 80-Min CD Capacity Bar -->
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

                <!-- Add YouTube Link Form -->
                <form @submit.prevent="handleAddSong" class="add-track-form">
                    <input v-model="inputUrl"
                           type="url"
                           class="peach-input"
                           placeholder="Paste YouTube song link..."
                           :disabled="store.isLoadingTrack"
                           required />
                    <button type="submit" class="peach-button" :disabled="store.isLoadingTrack">
                        {{ store.isLoadingTrack ? 'Fetching...' : '+ Add Track' }}
                    </button>
                </form>

                <p v-if="store.errorMessage" class="error-text">{{ store.errorMessage }}</p>

                <!-- Track List -->
                <div class="track-list">
                    <h3>Tracklist ({{ store.tracks.length }})</h3>

                    <div v-if="store.tracks.length === 0" class="empty-state">
                        No tracks added yet. Paste a YouTube link above to begin!
                    </div>

                    <div v-for="(track, index) in store.tracks"
                         :key="track.id"
                         class="track-item">
                        <span class="track-number">{{ (index + 1).toString().padStart(2, '0') }}</span>
                        <img :src="track.thumbnailUrl" alt="cover" class="track-thumb" />
                        <div class="track-info">
                            <span class="track-title">{{ track.title }}</span>
                            <span class="track-artist">{{ track.artist }}</span>
                        </div>
                        <button @click="store.removeTrack(track.id)" class="remove-btn" title="Remove track">✕</button>
                    </div>
                </div>
            </section>
        </main>
    </div>
</template>

<style>
    /* Peach Theme Palette */
    body {
        margin: 0;
        background-color: #fdf2f0;
        font-family: 'Courier New', Courier, monospace;
        color: #3d2b28;
    }

    .app-root {
        min-height: 100vh;
    }

    .momocider-station {
        max-width: 680px;
        margin: 0 auto;
        padding: 2.5rem 1rem;
        animation: fadeIn 0.8s ease-in-out;
    }

    .station-header {
        text-align: center;
        margin-bottom: 2rem;
    }

        .station-header h1 {
            font-size: 2.8rem;
            margin: 0;
            color: #e06d53;
            letter-spacing: 1px;
        }

    .subtitle {
        margin-top: 0.3rem;
        color: #936357;
        font-size: 0.9rem;
    }

    /* Peach Station Card */
    .card {
        background: #fff8f6;
        border: 3px solid #3d2b28;
        border-radius: 16px;
        padding: 1.8rem;
        box-shadow: 8px 8px 0px #f4c2b2;
    }

    .playlist-title-wrapper {
        margin-bottom: 1.5rem;
    }

        .playlist-title-wrapper label {
            display: block;
            font-weight: bold;
            font-size: 0.85rem;
            margin-bottom: 0.4rem;
            color: #7a4336;
        }

    .peach-input {
        width: 100%;
        box-sizing: border-box;
        padding: 0.75rem 1rem;
        border: 2px solid #3d2b28;
        border-radius: 8px;
        background: #ffffff;
        font-family: inherit;
        font-size: 0.95rem;
        color: #3d2b28;
        outline: none;
    }

        .peach-input:focus {
            border-color: #e06d53;
            box-shadow: 0 0 0 3px #fce4ec;
        }

    .title-input {
        font-weight: bold;
        font-size: 1.1rem;
    }

    /* Capacity Bar */
    .capacity-container {
        background: #fbeee8;
        border: 2px dashed #e0a394;
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
        color: #5c3a31;
    }

    .progress-bar-bg {
        width: 100%;
        height: 16px;
        background: #ead2c9;
        border-radius: 8px;
        overflow: hidden;
        border: 1px solid #3d2b28;
    }

    .progress-bar-fill {
        height: 100%;
        background: linear-gradient(90deg, #ffb7b2, #e06d53);
        transition: width 0.4s ease;
    }

        .progress-bar-fill.over-limit {
            background: #d90429;
        }

    .warning-text {
        color: #d90429;
        font-size: 0.8rem;
        margin: 0.5rem 0 0 0;
        font-weight: bold;
    }

    /* Track Input Form */
    .add-track-form {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .peach-button {
        background: #ffb7b2;
        border: 2px solid #3d2b28;
        border-radius: 8px;
        padding: 0.75rem 1.2rem;
        font-family: inherit;
        font-weight: bold;
        color: #3d2b28;
        cursor: pointer;
        white-space: nowrap;
        box-shadow: 3px 3px 0px #3d2b28;
        transition: transform 0.1s ease, box-shadow 0.1s ease;
    }

        .peach-button:hover {
            background: #ffa49e;
        }

        .peach-button:active {
            transform: translate(2px, 2px);
            box-shadow: 1px 1px 0px #3d2b28;
        }

    /* Track List */
    .track-list {
        margin-top: 1.5rem;
    }

        .track-list h3 {
            margin-bottom: 0.8rem;
            color: #5c3a31;
            font-size: 1rem;
        }

    .empty-state {
        text-align: center;
        padding: 2rem 1rem;
        color: #a06b5e;
        font-style: italic;
        font-size: 0.85rem;
    }

    .track-item {
        display: flex;
        align-items: center;
        gap: 0.8rem;
        background: #ffffff;
        border: 2px solid #3d2b28;
        border-radius: 8px;
        padding: 0.6rem;
        margin-bottom: 0.6rem;
    }

    .track-number {
        font-weight: bold;
        color: #e06d53;
    }

    .track-thumb {
        width: 44px;
        height: 44px;
        object-fit: cover;
        border-radius: 4px;
        border: 1px solid #3d2b28;
    }

    .track-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .track-title {
        font-weight: bold;
        font-size: 0.85rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .track-artist {
        font-size: 0.75rem;
        color: #7a4336;
    }

    .remove-btn {
        background: transparent;
        border: none;
        color: #a06b5e;
        font-size: 1.1rem;
        cursor: pointer;
        padding: 0.2rem 0.5rem;
    }

        .remove-btn:hover {
            color: #d90429;
        }

    .error-text {
        color: #d90429;
        font-size: 0.85rem;
        margin-top: 0.5rem;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(8px);
        }

        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>