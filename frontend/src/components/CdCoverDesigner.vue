<script setup lang="ts">
    import { ref } from 'vue';
    import { usePlaylistStore } from '../stores/playlist';

    const store = usePlaylistStore();
    const isOpen = ref(false);

    const themes = [
        { id: 'forest', name: 'Vintage Forest', bg: 'linear-gradient(135deg, #7f9a84 0%, #a2b7a5 40%, #c2a8a8 100%)' },
        { id: 'peach', name: 'Peach Cider', bg: 'linear-gradient(135deg, #ffb7b2 0%, #ffdac1 50%, #e2b0a2 100%)' },
        { id: 'lavender', name: 'Dream Lavender', bg: 'linear-gradient(135deg, #c7ceea 0%, #e2f0cb 50%, #b5ead7 100%)' },
        { id: 'midnight', name: 'Midnight Garden', bg: 'linear-gradient(135deg, #2b3a41 0%, #3a4f59 50%, #1e282d 100%)' }
    ];

    const icons = ['🌸', '🦋', '🌿', '🌼', '🍑', '✨', '🌙', '🍓'];

    function handlePrintBooklet() {
        window.print();
    }
</script>

<template>
    <div class="customizer-trigger-wrapper">
        <button class="open-modal-btn" @click="isOpen = true">
            🎨 Customize Cover & Print Booklet
        </button>
    </div>

    <!-- Modal Backdrop & Window -->
    <Teleport to="body">
        <div v-if="isOpen" class="modal-overlay" @click.self="isOpen = false">
            <div class="modal-content">
                <header class="modal-header">
                    <h3>🎨 Booklet & Cover Customizer</h3>
                    <button class="close-btn" @click="isOpen = false">✕</button>
                </header>

                <div class="designer-layout">
                    <!-- Live Preview -->
                    <div class="preview-wrapper">
                        <div class="cover-preview" :class="`theme-${store.coverTheme}`">
                            <div class="decorative-icon left-icon">{{ store.coverIcon }}</div>
                            <div class="decorative-icon right-icon">{{ store.coverIcon }}</div>
                            
                            <div class="cover-text">
                                <span class="badge">CD MIXTAPE</span>
                                <h2>{{ store.playlistName || 'Untitled Mixtape' }}</h2>
                                <p>{{ store.subtitle }}</p>
                            </div>
                        </div>
                        <p class="preview-caption">✦ Jewel Case Insert Preview ✦</p>
                    </div>

                    <!-- Controls -->
                    <div class="controls-wrapper">
                        <div class="control-group">
                            <label>Color Palette:</label>
                            <div class="palette-options">
                                <button v-for="t in themes"
                                        :key="t.id"
                                        class="theme-btn"
                                        :class="{ active: store.coverTheme === t.id }"
                                        :style="{ background: t.bg }"
                                        :title="t.name"
                                        @click="store.coverTheme = t.id as any"></button>
                            </div>
                        </div>

                        <div class="control-group">
                            <label>Cover Emblem:</label>
                            <div class="icon-options">
                                <button v-for="icon in icons"
                                        :key="icon"
                                        class="icon-btn"
                                        :class="{ active: store.coverIcon === icon }"
                                        @click="store.coverIcon = icon">
                                    {{ icon }}
                                </button>
                            </div>
                        </div>

                        <div class="control-group">
                            <label for="coverSubtitle">Subtitle / Dedication:</label>
                            <input id="coverSubtitle"
                                   v-model="store.subtitle"
                                   type="text"
                                   class="modal-input"
                                   placeholder="e.g. Summer '26 Memories" />
                        </div>

                        <div class="modal-actions">
                            <button @click="handlePrintBooklet" class="print-btn">
                                🖨️ Print Physical Booklet
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<style scoped>
    .customizer-trigger-wrapper {
        margin-top: 1.5rem;
        text-align: center;
    }

    .open-modal-btn {
        background: #38433a;
        border: 2px dashed #526355;
        border-radius: 8px;
        padding: 0.75rem 1.2rem;
        color: #c0d1c3;
        font-family: inherit;
        font-size: 0.85rem;
        font-weight: bold;
        cursor: pointer;
        width: 100%;
        transition: all 0.2s ease;
    }

    .open-modal-btn:hover {
        background: #424f44;
        border-color: #a3c9a8;
        color: #ffffff;
    }

    /* Modal Overlay */
    .modal-overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.75);
        backdrop-filter: blur(4px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 200;
        padding: 1rem;
    }

    .modal-content {
        background: #2b332d;
        border: 2px solid #526355;
        border-radius: 12px;
        padding: 1.5rem;
        max-width: 580px;
        width: 100%;
        box-shadow: 0 20px 40px rgba(0,0,0,0.5);
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.2rem;
        border-bottom: 1px solid #38433a;
        padding-bottom: 0.8rem;
    }

    .modal-header h3 {
        margin: 0;
        color: #e5ece6;
        font-size: 1.1rem;
    }

    .close-btn {
        background: none;
        border: none;
        color: #8fa393;
        font-size: 1.2rem;
        cursor: pointer;
    }

    .close-btn:hover {
        color: #ff7b7b;
    }

    .designer-layout {
        display: flex;
        gap: 1.5rem;
        flex-wrap: wrap;
    }

    .preview-wrapper {
        flex: 0 0 180px;
        text-align: center;
    }

    .cover-preview {
        width: 180px;
        height: 180px;
        border-radius: 8px;
        border: 2px dashed #e8d0c5;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        overflow: hidden;
    }

    .theme-forest { background: linear-gradient(135deg, #7f9a84 0%, #a2b7a5 40%, #c2a8a8 100%); }
    .theme-peach { background: linear-gradient(135deg, #ffb7b2 0%, #ffdac1 50%, #e2b0a2 100%); }
    .theme-lavender { background: linear-gradient(135deg, #c7ceea 0%, #e2f0cb 50%, #b5ead7 100%); }
    .theme-midnight { background: linear-gradient(135deg, #2b3a41 0%, #3a4f59 50%, #1e282d 100%); }

    .decorative-icon {
        position: absolute;
        font-size: 1.1rem;
        opacity: 0.6;
    }
    .left-icon { top: 8px; left: 8px; }
    .right-icon { bottom: 8px; right: 8px; }

    .cover-text {
        background: rgba(253, 248, 243, 0.9);
        padding: 0.6rem 0.8rem;
        border-radius: 6px;
        width: 80%;
        text-align: center;
    }

    .badge {
        font-size: 0.5rem;
        letter-spacing: 1px;
        font-weight: bold;
        color: #718073;
    }

    .cover-text h2 {
        margin: 0.2rem 0;
        font-size: 0.9rem;
        color: #382c28;
        font-family: Georgia, serif;
    }

    .cover-text p {
        margin: 0;
        font-size: 0.6rem;
        color: #63534e;
    }

    .preview-caption {
        font-size: 0.7rem;
        color: #8fa393;
        margin-top: 0.5rem;
    }

    .controls-wrapper {
        flex: 1;
        min-width: 220px;
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
    }

    .control-group label {
        display: block;
        font-size: 0.75rem;
        font-weight: bold;
        color: #b3c7b7;
        margin-bottom: 0.3rem;
    }

    .palette-options, .icon-options {
        display: flex;
        gap: 0.4rem;
        flex-wrap: wrap;
    }

    .theme-btn {
        width: 28px;
        height: 28px;
        border-radius: 50%;
        border: 2px solid #1e2420;
        cursor: pointer;
    }

    .theme-btn.active {
        border-color: #ffffff;
        transform: scale(1.1);
    }

    .icon-btn {
        background: #38433a;
        border: 1px solid #1e2420;
        border-radius: 6px;
        padding: 0.2rem 0.5rem;
        cursor: pointer;
    }

    .icon-btn.active {
        background: #7f9a84;
    }

    .modal-input {
        width: 100%;
        box-sizing: border-box;
        padding: 0.5rem;
        border-radius: 6px;
        border: 1px solid #1e2420;
        background: #38433a;
        color: #fff;
        font-family: inherit;
    }

    .print-btn {
        width: 100%;
        background: #7f9a84;
        border: none;
        border-radius: 6px;
        padding: 0.6rem;
        color: #fff;
        font-weight: bold;
        cursor: pointer;
        margin-top: 0.5rem;
    }
</style>