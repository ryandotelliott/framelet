import { invoke } from '@tauri-apps/api/core';

let isSelecting = false;
let startX = 0;
let startY = 0;
let currentX = 0;
let currentY = 0;

// Get the DPI scaling factor
const dpiScale = window.devicePixelRatio || 1;

const overlay = document.getElementById('selection-overlay') as HTMLElement;
const coordinates = document.getElementById('coordinates') as HTMLElement;
const cancelBtn = document.getElementById('cancel-btn') as HTMLButtonElement;

// ----- Event bindings -----
document.addEventListener('mousedown', startSelection);
document.addEventListener('mousemove', updateSelection);
document.addEventListener('mouseup', endSelection);
document.addEventListener('keydown', handleKeyDown);
cancelBtn.addEventListener('click', cancelSelection);

document.addEventListener('visibilitychange', () => {
  if (!document.hidden) {
    resetState();
  }
});

function startSelection(e: MouseEvent) {
  isSelecting = true;
  // Apply DPI scaling to get actual screen coordinates
  startX = Math.round(e.pageX * dpiScale);
  startY = Math.round(e.pageY * dpiScale);
  currentX = startX;
  currentY = startY;

  // Use unscaled coordinates for visual display
  overlay.style.left = `${e.pageX}px`;
  overlay.style.top = `${e.pageY}px`;
  overlay.style.width = '0px';
  overlay.style.height = '0px';
  overlay.classList.remove('hidden');
  coordinates.classList.remove('hidden');
  updateCoordinatesDisplay();
}

function updateSelection(e: MouseEvent) {
  if (!isSelecting) return;

  // Apply DPI scaling to get actual screen coordinates
  currentX = Math.round(e.pageX * dpiScale);
  currentY = Math.round(e.pageY * dpiScale);
  // Use unscaled coordinates for visual display
  const displayLeft = Math.min(startX / dpiScale, currentX / dpiScale);
  const displayTop = Math.min(startY / dpiScale, currentY / dpiScale);
  const displayWidth = Math.abs(currentX / dpiScale - startX / dpiScale);
  const displayHeight = Math.abs(currentY / dpiScale - startY / dpiScale);

  overlay.style.left = `${displayLeft}px`;
  overlay.style.top = `${displayTop}px`;
  overlay.style.width = `${displayWidth}px`;
  overlay.style.height = `${displayHeight}px`;

  updateCoordinatesDisplay();
}

function updateCoordinatesDisplay() {
  const left = Math.min(startX, currentX);
  const top = Math.min(startY, currentY);
  const width = Math.abs(currentX - startX);
  const height = Math.abs(currentY - startY);

  coordinates.textContent = `X: ${left}, Y: ${top}, Width: ${width}, Height: ${height} (DPI: ${dpiScale})`;
}

async function endSelection() {
  if (!isSelecting) return;
  isSelecting = false;

  const left = Math.min(startX, currentX);
  const top = Math.min(startY, currentY);
  const width = Math.abs(currentX - startX);
  const height = Math.abs(currentY - startY);

  if (width > 10 && height > 10) {
    const regionData = { x: left, y: top, width, height };
    try {
      await invoke('region_selected', { coordinates: regionData });
      await cancelSelection();
    } catch (error) {
      console.error('Error sending region coordinates:', error);
      await cancelSelection();
    }
  } else {
    await cancelSelection();
  }
}

async function cancelSelection() {
  resetState();
  try {
    await invoke('close_region_selector');
  } catch (error) {
    console.error('Error closing region selector:', error);
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    cancelSelection();
  }
}

function resetState() {
  isSelecting = false;
  overlay.classList.add('hidden');
  coordinates.classList.add('hidden');
  startX = startY = currentX = currentY = 0;
}
