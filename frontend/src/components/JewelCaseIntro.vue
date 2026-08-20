<script setup lang="ts">
    import { ref } from 'vue';

    const emit = defineEmits<{
        (e: 'animation-complete'): void
    }>();

    const isOpening = ref(false);
    const isSpinning = ref(false);
    const isZooming = ref(false);

    function triggerOpen() {
        if (isOpening.value) return;

        // Step 1: Swing open the plastic jewel case lid (slower)
        isOpening.value = true;

        // Step 2: CD starts spinning midway through opening
        setTimeout(() => {
            isSpinning.value = true;
        }, 700);

        // Step 3: Zoom deep in + morph background color
        setTimeout(() => {
            isZooming.value = true;
        }, 2200);

        // Step 4: Complete intro & hand off to station UI
        setTimeout(() => {
            emit('animation-complete');
        }, 4200);
    }
</script>

<template>
    <div class="intro-overlay" :class="{ 'zoom-into-center': isZooming }">
        <div class="jewel-case-wrapper"
             :class="{ 'zoom-in': isZooming }"
             @click="triggerOpen">
            <div class="jewel-case">
                <!-- CD Tray (Botanical Forest Metallic) -->
                <div class="cd-tray">
                    <div class="tray-left-spine"></div>

                    <!-- Plastic Retention Clips -->
                    <div class="tab tab-top-left"></div>
                    <div class="tab tab-top-right"></div>
                    <div class="tab tab-bottom-left"></div>
                    <div class="tab tab-bottom-right"></div>

                    <!-- Dreamy Botanical CD Disc -->
                    <div class="cd-disc" :class="{ 'spin-active': isSpinning }">
                        <!-- Whimsical Botanical & Butterfly Accents -->
                        <div class="botanical-ring">
                            <span class="botanical-element element-1">🦋</span>
                            <span class="botanical-element element-2">🌸</span>
                            <span class="botanical-element element-3">🌿</span>
                            <span class="botanical-element element-4">🌼</span>
                            <span class="botanical-element element-5">🦋</span>
                            <span class="botanical-element element-6">🌷</span>
                        </div>

                        <!-- Inner Clear Ring with Dark Forest Hub -->
                        <div class="inner-clear-ring">
                            <div class="cd-center-hub">
                                <span class="disc-text">MOMOCIDER</span>
                                <div class="center-hole"></div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Glass Front Lid with Whimsical Garden Booklet -->
                <div class="cd-lid" :class="{ 'lid-open': isOpening }">
                    <div class="glass-glare"></div>
                    <div class="hinge top-hinge"></div>
                    <div class="hinge bottom-hinge"></div>

                    <div class="album-art">
                        <div class="garden-background">
                            <span class="art-floating art-1">🦋</span>
                            <span class="art-floating art-2">🌸</span>
                            <span class="art-floating art-3">🌿</span>
                            <span class="art-floating art-4">🌺</span>
                            <span class="art-floating art-5">🐝</span>
                            <span class="art-floating art-6">✨</span>
                        </div>

                        <div class="artwork-content">
                            <h2 class="cover-title">Momocider</h2>
                            <p class="cover-subtitle">Digital Mixtape Station</p>
                        </div>
                    </div>
                </div>
            </div>

            <p v-if="!isOpening" class="click-prompt">✦ Click jewel case to open ✦</p>
        </div>
    </div>
</template>

<style scoped>
    @import url('https://fonts.googleapis.com/css2?family=Playfair+Display:ital,wght@1,500&display=swap');

    .intro-overlay {
        position: fixed;
        inset: 0;
        background: radial-gradient(circle, #f3ece1 0%, #e3d3c1 100%);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 100;
        transition: background-color 1.8s cubic-bezier(0.25, 1, 0.5, 1), opacity 1.8s cubic-bezier(0.25, 1, 0.5, 1);
        overflow: hidden;
    }

    .intro-overlay.zoom-into-center {
        background-color: #38433a;
        opacity: 0;
        pointer-events: none;
    }

    .jewel-case-wrapper {
        position: relative;
        perspective: 1600px;
        cursor: pointer;
        text-align: center;
        transform-origin: center center;
        transition: transform 2.2s cubic-bezier(0.25, 1, 0.5, 1);
    }

    .jewel-case-wrapper.zoom-in {
        transform: scale(26) translate(12px, 0px);
    }

    /* Outer Glass Case */
    .jewel-case {
        position: relative;
        width: 420px;
        height: 372px;
        background: rgba(255, 255, 255, 0.35);
        border: 2px solid rgba(255, 255, 255, 0.85);
        border-radius: 8px;
        box-shadow: 0 25px 50px rgba(60, 75, 65, 0.25), inset 0 0 0 1px rgba(255, 255, 255, 0.6);
        transform-style: preserve-3d;
    }

    /* Vintage Forest Green Tray */
    .cd-tray {
        position: absolute;
        inset: 8px;
        background: linear-gradient(135deg, #718073 0%, #4a574d 50%, #343d36 100%);
        border-radius: 6px;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: inset 0 0 16px rgba(0, 0, 0, 0.35);
        overflow: hidden;
    }

    /* Left Spine Section */
    .tray-left-spine {
        position: absolute;
        left: 0;
        top: 0;
        bottom: 0;
        width: 42px;
        background: linear-gradient(90deg, #3d473f, #637366 80%, #48544a 100%);
        border-right: 2px solid #2d362f;
        box-shadow: inset -2px 0 5px rgba(0,0,0,0.2);
    }

    /* Retention Tabs */
    .tab {
        position: absolute;
        width: 36px;
        height: 12px;
        background: #556358;
        border-radius: 10px;
        box-shadow: inset 0 1px 2px rgba(0,0,0,0.4);
    }

    .tab-top-left { top: 6px; left: 70px; }
    .tab-top-right { top: 6px; right: 24px; }
    .tab-bottom-left { bottom: 6px; left: 70px; }
    .tab-bottom-right { bottom: 6px; right: 24px; }

    /* Iridescent Metallic Disc */
    .cd-disc {
        position: relative;
        width: 348px;
        height: 348px;
        margin-left: 24px;
        border-radius: 50%;
        background: conic-gradient( from 30deg, #d1e0d3, #f2dede, #b8cbd0, #e8dad6, #c5d8c8, #f0dfdd, #d1e0d3 );
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 4px 18px rgba(0, 0, 0, 0.25), inset 0 0 18px rgba(255, 255, 255, 0.85);
    }

    .cd-disc.spin-active {
        animation: spinCd 2s linear infinite;
    }

    /* Botanical Ring on CD Surface */
    .botanical-ring {
        position: absolute;
        inset: 0;
        border-radius: 50%;
        pointer-events: none;
    }

    .botanical-element {
        position: absolute;
        font-size: 1.3rem;
        opacity: 0.85;
        filter: drop-shadow(0 2px 3px rgba(0,0,0,0.15));
    }

    .element-1 { top: 26px; left: 50%; transform: translateX(-50%); }
    .element-2 { bottom: 26px; left: 50%; transform: translateX(-50%); }
    .element-3 { left: 26px; top: 50%; transform: translateY(-50%); }
    .element-4 { right: 26px; top: 50%; transform: translateY(-50%); }
    .element-5 { top: 72px; right: 72px; transform: rotate(20deg); }
    .element-6 { bottom: 72px; left: 72px; transform: rotate(-20deg); }

    /* Inner Ring */
    .inner-clear-ring {
        width: 156px;
        height: 156px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.4);
        border: 1px solid rgba(255, 255, 255, 0.85);
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: inset 0 0 8px rgba(0,0,0,0.08);
    }

    /* Dark Forest Center Hub */
    .cd-center-hub {
        width: 115px;
        height: 115px;
        border-radius: 50%;
        background: #38433a;
        border: 2px solid rgba(255, 255, 255, 0.8);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        box-shadow: 0 2px 8px rgba(0,0,0,0.3);
    }

    .disc-text {
        font-size: 0.75rem;
        font-family: 'Courier New', Courier, monospace;
        font-weight: 700;
        letter-spacing: 1.4px;
        color: #d1e0d3;
        margin-bottom: 2px;
    }

    .center-hole {
        width: 34px;
        height: 34px;
        border-radius: 50%;
        background: #283029;
        border: 2px solid #505d52;
    }

    /* Acrylic Glass Lid */
    .cd-lid {
        position: absolute;
        inset: 0;
        background: linear-gradient( 135deg, rgba(255, 255, 255, 0.45) 0%, rgba(255, 255, 255, 0.15) 50%, rgba(255, 255, 255, 0.3) 100% );
        border: 1.5px solid rgba(255, 255, 255, 0.9);
        border-radius: 8px;
        transform-origin: left center;
        /* Slowed down lid opening from 1.2s to 1.8s */
        transition: transform 1.8s cubic-bezier(0.25, 1, 0.4, 1);
        backface-visibility: hidden;
        padding: 10px 10px 10px 22px;
    }

    .cd-lid.lid-open {
        transform: rotateY(-150deg);
    }

    .glass-glare {
        position: absolute;
        inset: 0;
        background: linear-gradient( 60deg, transparent 40%, rgba(255, 255, 255, 0.35) 50%, transparent 60% );
        pointer-events: none;
        border-radius: 8px;
    }

    .hinge {
        position: absolute;
        left: 5px;
        width: 10px;
        height: 22px;
        background: rgba(255, 255, 255, 0.85);
        border: 1px solid rgba(150, 150, 150, 0.4);
        border-radius: 2px;
        z-index: 5;
    }

    .top-hinge { top: 40px; }
    .bottom-hinge { bottom: 40px; }

    /* Whimsical Forest Booklet Art */
    .album-art {
        position: relative;
        width: 100%;
        height: 100%;
        background: linear-gradient(135deg, #7f9a84 0%, #a2b7a5 40%, #c2a8a8 100%);
        border: 2px dashed #e8d0c5;
        box-sizing: border-box;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 6px;
        overflow: hidden;
        box-shadow: inset 0 0 20px rgba(0,0,0,0.15);
    }

    .garden-background {
        position: absolute;
        inset: 0;
        pointer-events: none;
    }

    .art-floating {
        position: absolute;
        font-size: 1.6rem;
        opacity: 0.7;
    }

    .art-1 { top: 16px; left: 20px; transform: rotate(-15deg); }
    .art-2 { top: 20px; right: 24px; transform: rotate(10deg); }
    .art-3 { bottom: 18px; left: 22px; transform: rotate(25deg); }
    .art-4 { bottom: 22px; right: 22px; transform: rotate(-10deg); }
    .art-5 { top: 50%; left: 16px; transform: translateY(-50%); font-size: 1.3rem; }
    .art-6 { top: 40%; right: 18px; transform: translateY(-50%); font-size: 1.3rem; }

    .artwork-content {
        position: relative;
        z-index: 2;
        text-align: center;
        background: rgba(253, 248, 243, 0.88);
        backdrop-filter: blur(2px);
        padding: 1.5rem 2.2rem;
        border-radius: 14px;
        border: 1px solid rgba(255, 255, 255, 0.9);
        box-shadow: 0 4px 18px rgba(45, 60, 50, 0.15);
    }

    .cover-title {
        margin: 0;
        font-family: 'Playfair Display', Georgia, serif;
        font-style: italic;
        font-size: 2.5rem;
        font-weight: 500;
        color: #382c28;
        letter-spacing: -0.5px;
    }

    .cover-subtitle {
        margin: 0.4rem 0 0 0;
        font-family: 'Courier New', Courier, monospace;
        font-size: 0.88rem;
        color: #63534e;
        letter-spacing: 0.5px;
    }

    .click-prompt {
        margin-top: 2rem;
        font-family: 'Courier New', Courier, monospace;
        font-size: 1rem;
        color: #7d6b63;
        font-weight: 600;
        animation: pulse 2s infinite;
    }

    @keyframes spinCd {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.35; }
    }
</style>