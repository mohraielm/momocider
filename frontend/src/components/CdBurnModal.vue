<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { usePlaylistStore } from '../stores/playlist';
import { getBurners, type BurnerInfo } from '../utils/tauri-ipc';

const store = usePlaylistStore();
const isOpen = ref(false);
const isBurning = ref(false);
const burners = ref<BurnerInfo[]>([]);
const selectedBurnerId = ref('');
const statusMessage = ref('');

onMounted(async () => {
    // Load available burners when modal is opened
    if (isOpen.value) {
        await loadBurners();
    }
});

async function loadBurners() {
    try {
        burners.value = await getBurners();
        if (burners.value.length > 0) {
            selectedBurnerId.value = burners.value[0].id;
            statusMessage.value = '';
        } else {
            selectedBurnerId.value = '';
            statusMessage.value = '❌ No optical drive detected. Please connect a CD/DVD writer and make sure it appears in Windows Device Manager.';
        }
    } catch (error: any) {
        selectedBurnerId.value = '';
        statusMessage.value = `❌ No optical drive detected. Please connect a CD/DVD writer and make sure it appears in Windows Device Manager. (${error})`;
    }
}

async function startBurn() {
    if (!selectedBurnerId.value) {
        statusMessage.value = '❌ Please select a burner first.';
        return;
    }

    if (store.tracks.length === 0) {
        statusMessage.value = '❌ Add at least one song before burning a CD.';
        return;
    }

    isBurning.value = true;
    statusMessage.value = '🔥 Burning CD...';

    try {
        await store.burnCdToDrive(selectedBurnerId.value);
        statusMessage.value = '✅ CD burned successfully!';
        setTimeout(() => {
            isOpen.value = false;
            statusMessage.value = '';
        }, 2000);
    } catch (error: any) {
        statusMessage.value = `❌ Burn failed: ${error}`;
    } finally {
        isBurning.value = false;
    }
}

function handleModalOpen() {
    isOpen.value = true;
    statusMessage.value = '';
    loadBurners();
}
</script>

<template>
    <div>
        <!-- Trigger Button -->
        <button class="export-btn burn-modal-trigger" @click="handleModalOpen" title="Open CD Burner">
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

                <!-- Burner Selection -->
                <div v-if="burners.length > 0" class="burner-selector">
                    <label for="burner-select">Select CD Burner:</label>
                    <select 
                        id="burner-select"
                        v-model="selectedBurnerId" 
                        class="peach-input burner-select"
                        :disabled="isBurning"
                    >
                        <option 
                            v-for="burner in burners" 
                            :key="burner.id" 
                            :value="burner.id"
                        >
                            {{ burner.name }}
                        </option>
                    </select>
                </div>

                <!-- Status Message -->
                <div v-if="statusMessage" class="status-message" :class="{ error: statusMessage.includes('❌') }">
                    {{ statusMessage }}
                </div>

                <div class="modal-actions">
                    <button class="peach-button cancel-btn" @click="isOpen = false" :disabled="isBurning">Cancel</button>
                    <button class="peach-button burn-btn" @click="startBurn" :disabled="isBurning || selectedBurnerId === ''" :title="selectedBurnerId ? 'Start burning' : 'Select a burner first'">
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
    max-width: 500px;
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
    margin-bottom: 0;
}
.burner-selector {
    margin: 1rem 0;
}
.burner-selector label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
    color: #c9d3cc;
}
.burner-select {
    width: 100%;
    padding: 0.6rem;
    background: #323b34;
    color: #e5ece6;
    border: 2px solid #526355;
    border-radius: 6px;
    font-family: 'Courier New', Courier, monospace;
}
.burner-select:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}
.status-message {
    padding: 0.8rem;
    margin: 1rem 0;
    border-radius: 6px;
    background: #323b34;
    border-left: 4px solid #76d776;
    font-size: 0.9rem;
}
.status-message.error {
    border-left-color: #ff6b6b;
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
.burn-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}
</style>