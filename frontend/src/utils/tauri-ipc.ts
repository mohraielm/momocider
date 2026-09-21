/**
 * Tauri IPC utilities for communicating with the Rust backend
 * Handles CD burning operations and device detection
 */

import { invoke } from '@tauri-apps/api/core';

export interface BurnerInfo {
    id: string;
    name: string;
    supports_cd: boolean;
    supports_dvd: boolean;
}

export interface Track {
    path: string;
    title: string;
}

/**
 * Get list of available CD burners on the system
 */
export async function getBurners(): Promise<BurnerInfo[]> {
    try {
        return await invoke('get_burners');
    } catch (error) {
        console.error('Failed to get burners:', error);
        throw error;
    }
}

/**
 * Burn audio CD with selected burner
 */
export async function burnCd(
    burnerId: string,
    tracks: Track[],
    playlistName: string
): Promise<string> {
    try {
        return await invoke('burn_cd', {
            burnerId,
            tracks,
            playlistName,
        });
    } catch (error) {
        console.error('Burn failed:', error);
        throw error;
    }
}

/**
 * Check burn status of a device
 */
export async function getBurnStatus(burnerId: string): Promise<any> {
    try {
        return await invoke('get_burn_status', { burnerId });
    } catch (error) {
        console.error('Failed to get burn status:', error);
        throw error;
    }
}

/**
 * Fetch track metadata from Node backend
 */
export async function fetchTrackInfo(url: string): Promise<any> {
    try {
        return await invoke('fetch_track_info', { url });
    } catch (error) {
        console.error('Failed to fetch track info:', error);
        throw error;
    }
}

/**
 * Export playlist as ZIP for burning
 */
export async function exportZip(tracks: any[], playlistName: string): Promise<string> {
    try {
        return await invoke('export_zip', { tracks, playlistName });
    } catch (error) {
        console.error('Export failed:', error);
        throw error;
    }
}
