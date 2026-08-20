<script setup lang="ts">
import { ref } from 'vue';
import { usePlaylistStore } from '../stores/playlist';

const store = usePlaylistStore();
const isOpen = ref(false);
const isBurning = ref(false);

async function startBurn() {
    isBurning.value = true;
    await store.burnCdToDrive();
    isBurning.value = false;
    isOpen.value = false;
}
</script>

<template>
    <div>
        <!-- Trigger Button -->
        <button class="export-btn burn-modal-trigger" @click="isOpen = true" title="Open CD Burner">
            🔥 Burn CD...
        </button>

        <!-- Modal Overlay -->
        <div v-if="isOpen" class="modal-overlay">
            <div class="modal-card">
                <h2>💿 Burn Physical CD</h2>
                <p class="modal-sub">Mixtape: <strong>{{ store.playlistName }}</strong></p>

                <div class="burn-status-box">
                    <p>Tracks: <strong>{{ store.tracks.length }}</strong></p>
                    <p>Total Time: <strong>{{ store.totalDurationFormatted }}</strong></p>
                    <p class="disc-warning">⚠️ Please ensure a blank CD-R is inserted into your disc drive before starting.</p>
                </div>

                <div class="modal-actions">
                    <button class="peach-button cancel-btn" @click="isOpen = false" :disabled="isBurning">Cancel</button>
                    <button class="peach-button burn-btn" @click="startBurn" :disabled="isBurning">
                        {{ isBurning ? '🔥 Burning Disc...' : '🚀 Start Burning' }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.burn-modal-trigger {
    background: #a86b5b !important;
    font-weight: bold;
}
.modal-overlay {
    position: fixed;
    top: 0; left: 0; width: 100%; height: 100%;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100;
}
.modal-card {
    background: #2b332d;
    border: 3px solid #1e2420;
    border-radius: 14px;
    padding: 2rem;
    width: 90%;
    max-width: 420px;
    box-shadow: 6px 6px 0px #1a211c;
    color: #e5ece6;
    font-family: 'Courier New', Courier, monospace;
}
.modal-card h2 {
    margin-top: 0;
    color: #e8c5b8;
}
.burn-status-box {
    background: #323b34;
    border: 2px dashed #526355;
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    font-size: 0.9rem;
}
.disc-warning {
    color: #ffbbbb;
    font-size: 0.8rem;
    margin-top: 0.8rem;
}
.modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.8rem;
    margin-top: 1.5rem;
}
.cancel-btn {
    background: #526355;
}
.burn-btn {
    background: #b86b5b;
}
</style>