<script setup lang="ts">
    import { ref } from 'vue';

    const emit = defineEmits<{
        (e: 'animation-complete'): void
    }>();

    const isOpening = ref(false);
    const isZooming = ref(false);

    function triggerOpen() {
        if (isOpening.value) return;

        // Phase 1: Gently swing open the lid (slower timing)
        isOpening.value = true;

        // Phase 2: Wait for lid to swing mostly open before beginning zoom (2.2s)
        setTimeout(() => {
            isZooming.value = true;
        }, 2200);

        // Phase 3: Complete transition after smooth deep zoom finishes (5.5s total)
        setTimeout(() => {
            emit('animation-complete');
        }, 5500);
    }
</script>

<template>
    <div class="intro-overlay" :class="{ 'fade-out': isZooming }">
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
                    <div class="cd-disc">
                        <!-- Whimsical Botanical & Butterfly Accents -->
                        <div class="botanical-ring">
                            <span class="botanical-element element-1">🦋</span>
                            <span class="botanical-element element-2">🌸</span>
                            <span class="botanical-element element-3">🌿</span>
                            <span class="botanical-element element-4">🌼</span>
                            <span class="botanical-element element-5">🦋</span>
                            <span class="botanical-element element-6">🌷</span>
                        </div>

                        <!-- Inner Soft Sage Center Ring Target -->
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
        /* Slower 2.5s backdrop fade */
        transition: opacity 2.5s cubic-bezier(0.22, 1, 0.36, 1);
        overflow: hidden;
    }

        .intro-overlay.fade-out {
            opacity: 0;
            pointer-events: none;
        }

    .jewel-case-wrapper {
        position: relative;
        perspective: 1600px;
        cursor: pointer;
        text-align: center;
        transform-origin: center center;
        /* Slower 3.2s zoom curve */
        transition: transform 3.2s cubic-bezier(0.22, 1, 0.36, 1);
    }

        .jewel-case-wrapper.zoom-in {
            transform: scale(15) translate(10px, 0px);
        }

    /* Outer Glass Case */
    .jewel-case {
        position: relative;
        width: 350px;
        height: 310px;
        background: rgba(255, 255, 255, 0.35);
        border: 2px solid rgba(255, 255, 255, 0.85);
        border-radius: 6px;
        box-shadow: 0 20px 40px rgba(60, 75, 65, 0.2), inset 0 0 0 1px rgba(255, 255, 255, 0.6);
        transform-style: preserve-3d;
    }

    /* Vintage Forest Green-Grey Tray */
    .cd-tray {
        position: absolute;
        inset: 6px 6px 6px 6px;
        background: linear-gradient(135deg, #718073 0%, #4a574d 50%, #343d36 100%);
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: inset 0 0 14px rgba(0, 0, 0, 0.35);
        overflow: hidden;
    }

    /* Left Spine Section */
    .tray-left-spine {
        position: absolute;
        left: 0;
        top: 0;
        bottom: 0;
        width: 36px;
        background: linear-gradient(90deg, #3d473f, #637366 80%, #48544a 100%);
        border-right: 2px solid #2d362f;
        box-shadow: inset -2px 0 5px rgba(0,0,0,0.2);
    }

    /* Retention Tabs */
    .tab {
        position: absolute;
        width: 30px;
        height: 10px;
        background: #556358;
        border-radius: 10px;
        box-shadow: inset 0 1px 2px rgba(0,0,0,0.4);
    }

    .tab-top-left {
        top: 4px;
        left: 60px;
    }

    .tab-top-right {
        top: 4px;
        right: 20px;
    }

    .tab-bottom-left {
        bottom: 4px;
        left: 60px;
    }

    .tab-bottom-right {
        bottom: 4px;
        right: 20px;
    }

    /* Iridescent Metallic Disc */
    .cd-disc {
        position: relative;
        width: 290px;
        height: 290px;
        margin-left: 20px;
        border-radius: 50%;
        background: conic-gradient( from 30deg, #d1e0d3, #f2dede, #b8cbd0, #e8dad6, #c5d8c8, #f0dfdd, #d1e0d3 );
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 4px 15px rgba(0, 0, 0, 0.25), inset 0 0 15px rgba(255, 255, 255, 0.85);
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
        font-size: 1.1rem;
        opacity: 0.85;
        filter: drop-shadow(0 2px 3px rgba(0,0,0,0.15));
    }

    .element-1 {
        top: 22px;
        left: 50%;
        transform: translateX(-50%);
    }

    .element-2 {
        bottom: 22px;
        left: 50%;
        transform: translateX(-50%);
    }

    .element-3 {
        left: 22px;
        top: 50%;
        transform: translateY(-50%);
    }

    .element-4 {
        right: 22px;
        top: 50%;
        transform: translateY(-50%);
    }

    .element-5 {
        top: 60px;
        right: 60px;
        transform: rotate(20deg);
    }

    .element-6 {
        bottom: 60px;
        left: 60px;
        transform: rotate(-20deg);
    }

    /* Inner Clear Ring */
    .inner-clear-ring {
        width: 130px;
        height: 130px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.5);
        border: 1px solid rgba(255, 255, 255, 0.85);
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: inset 0 0 8px rgba(0,0,0,0.08);
    }

    /* Sage Center Hub (Center Target) */
    .cd-center-hub {
        width: 96px;
        height: 96px;
        border-radius: 50%;
        background: #a3c9a8;
        border: 2px solid #ffffff;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        box-shadow: 0 2px 6px rgba(0,0,0,0.15);
    }

    .disc-text {
        font-size: 0.65rem;
        font-family: 'Courier New', Courier, monospace;
        font-weight: 700;
        letter-spacing: 1.2px;
        color: #2c4230;
        margin-bottom: 2px;
    }

    .center-hole {
        width: 28px;
        height: 28px;
        border-radius: 50%;
        background: #f3ece1;
        border: 2px solid #d0e3d2;
    }

    /* Acrylic Glass Lid */
    .cd-lid {
        position: absolute;
        inset: 0;
        background: linear-gradient( 135deg, rgba(255, 255, 255, 0.45) 0%, rgba(255, 255, 255, 0.15) 50%, rgba(255, 255, 255, 0.3) 100% );
        border: 1.5px solid rgba(255, 255, 255, 0.9);
        border-radius: 6px;
        transform-origin: left center;
        /* Slower 2.6s lid opening swing */
        transition: transform 2.6s cubic-bezier(0.22, 1, 0.36, 1);
        backface-visibility: hidden;
        padding: 8px 8px 8px 18px;
    }

        .cd-lid.lid-open {
            transform: rotateY(-150deg);
        }

    .glass-glare {
        position: absolute;
        inset: 0;
        background: linear-gradient( 60deg, transparent 40%, rgba(255, 255, 255, 0.35) 50%, transparent 60% );
        pointer-events: none;
        border-radius: 6px;
    }

    .hinge {
        position: absolute;
        left: 4px;
        width: 8px;
        height: 18px;
        background: rgba(255, 255, 255, 0.85);
        border: 1px solid rgba(150, 150, 150, 0.4);
        border-radius: 2px;
        z-index: 5;
    }

    .top-hinge {
        top: 35px;
    }

    .bottom-hinge {
        bottom: 35px;
    }

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
        border-radius: 4px;
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
        font-size: 1.4rem;
        opacity: 0.7;
    }

    .art-1 {
        top: 12px;
        left: 16px;
        transform: rotate(-15deg);
    }

    .art-2 {
        top: 16px;
        right: 20px;
        transform: rotate(10deg);
    }

    .art-3 {
        bottom: 14px;
        left: 18px;
        transform: rotate(25deg);
    }

    .art-4 {
        bottom: 18px;
        right: 18px;
        transform: rotate(-10deg);
    }

    .art-5 {
        top: 50%;
        left: 12px;
        transform: translateY(-50%);
        font-size: 1.1rem;
    }

    .art-6 {
        top: 40%;
        right: 14px;
        transform: translateY(-50%);
        font-size: 1.1rem;
    }

    .artwork-content {
        position: relative;
        z-index: 2;
        text-align: center;
        background: rgba(253, 248, 243, 0.88);
        backdrop-filter: blur(2px);
        padding: 1.2rem 1.8rem;
        border-radius: 12px;
        border: 1px solid rgba(255, 255, 255, 0.9);
        box-shadow: 0 4px 15px rgba(45, 60, 50, 0.15);
    }

    .cover-title {
        margin: 0;
        font-family: 'Playfair Display', Georgia, serif;
        font-style: italic;
        font-size: 2.1rem;
        font-weight: 500;
        color: #382c28;
        letter-spacing: -0.5px;
    }

    .cover-subtitle {
        margin: 0.3rem 0 0 0;
        font-family: 'Courier New', Courier, monospace;
        font-size: 0.78rem;
        color: #63534e;
        letter-spacing: 0.5px;
    }

    .click-prompt {
        margin-top: 1.8rem;
        font-family: 'Courier New', Courier, monospace;
        color: #7d6b63;
        font-weight: 600;
        animation: pulse 2s infinite;
    }

    @keyframes pulse {
        0%, 100% {
            opacity: 1;
        }

        50% {
            opacity: 0.35;
        }
    }
</style>