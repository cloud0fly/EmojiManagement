<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { readDir } from "@tauri-apps/plugin-fs";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface Meme {
  name: string;
  src: string;
  rawPath: string;
}

const folderPath = ref("");
const images = ref<Meme[]>([]);
const searchQuery = ref("");
const showToastFlg = ref(false);
const showToastMsg = ref("");
const isAlwaysOnTop = ref(false);

// 匹配文件名
const filteredImages = computed(() => {
  const query = searchQuery.value.toLowerCase();
  return images.value.filter((img) => img.name.toLowerCase().includes(query));
});

// 扫描路径
const scanFolder = async (path: string) => {
  try {
    const entries = await readDir(path);
    images.value = entries
      .filter(
        (entry) =>
          !entry.isDirectory && entry.name.match(/\.(png|jpg|gif|jpeg|webp)$/i),
      ) // 仅保留图片格式文件
      .map((entry) => {
        const fullPath = `${path}\\${entry.name}`;
        return {
          name: entry.name,
          src: convertFileSrc(fullPath),
          rawPath: fullPath,
        };
      });
    folderPath.value = path;
    localStorage.setItem("last_meme_folder", path);
  } catch (err) {
    console.error("扫描失败:", err);
    // 路径失效
    localStorage.removeItem("last_meme_folder");
  }
};

onMounted(() => {
  const savedPath = localStorage.getItem("last_meme_folder");
  if (savedPath) {
    console.log("正在自动加载上一次使用的路径...", savedPath);
    scanFolder(savedPath);
  }
});

let toastTimer: number | null = null;
const triggerToast = (msg: string) => {
  if (toastTimer) {
    clearTimeout(toastTimer);
  }

  showToastMsg.value = msg;
  showToastFlg.value = true;

  // 2秒后自动隐藏
  toastTimer = window.setTimeout(() => {
    showToastFlg.value = false;
    toastTimer = null;
  }, 2000);
};

const selectFolder = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "请选择存放目录",
  });

  if (selected && typeof selected === "string") {
    await scanFolder(selected);
  }
};

const copyImage = async (filePath: string) => {
  try {
    await invoke("copy_image", { path: filePath });
    triggerToast("已成功存入剪贴板");
  } catch (err) {
    console.error("复制失败:", err);
  }
};

const toggleAlwaysOnTop = async () => {
  const appWindow = getCurrentWindow();
  isAlwaysOnTop.value = !isAlwaysOnTop.value;
  await appWindow.setAlwaysOnTop(isAlwaysOnTop.value);
  triggerToast(isAlwaysOnTop.value ? "已置顶窗口" : "已取消置顶");
};
</script>

<template>
  <div class="app-container">
    <Transition name="toast">
      <div v-if="showToastFlg" class="toast-container">
        <div class="toast-content">
          {{ showToastMsg }}
        </div>
      </div>
    </Transition>
    <header class="toolbar">
      <div class="search-wrapper">
        <span class="search-icon">🔍</span>
        <input
          v-model="searchQuery"
          placeholder="搜索表情包..."
          class="search-input"
        />
        <button v-if="searchQuery" class="clear-btn" @click="searchQuery = ''">
          ✕
        </button>
      </div>

      <div class="divider"></div>

      <div class="actions">
        <button
          class="icon-btn"
          :class="{ active: isAlwaysOnTop }"
          @click="toggleAlwaysOnTop"
          title="窗口置顶"
        >
          <span class="icon">📌</span>
        </button>

        <div class="divider"></div>
        <button class="primary-btn" @click="selectFolder">
          <span class="icon">📁</span>
          <span>扫描文件夹</span>
        </button>
        <div v-if="folderPath" class="path-badge" :title="folderPath">
          {{ folderPath.split("\\").pop() }}
        </div>
      </div>
    </header>

    <main class="content">
      <div v-if="images.length === 0" class="empty-state">
        <p>还没有表情包，点击上方按钮开始扫描</p>
      </div>

      <div class="meme-grid">
        <div
          v-for="img in filteredImages"
          :key="img.rawPath"
          class="meme-card"
          @click="copyImage(img.rawPath)"
        >
          <img :src="img.src" loading="lazy" :title="img.name" />
          <div class="meme-info">{{ img.name }}</div>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
/* 基础全局样式 */
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  background-color: #f9f9f9;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

/* Toast 容器：负责定位 */
.toast-container {
  position: fixed;
  bottom: 50px; /* 距离底部高度 */
  left: 50%;
  transform: translateX(-50%);
  z-index: 9999;
  pointer-events: none; /* 防止遮挡点击 */
}

/* Toast 内容：负责外观 */
.toast-content {
  background: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(8px);
  color: white;
  padding: 12px 24px;
  border-radius: 50px;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  white-space: nowrap;
}

/* Vue Transition 动画效果 */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.4s cubic-bezier(0.18, 0.89, 0.32, 1.28);
}

.toast-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.toolbar {
  display: flex;
  align-items: center;
  padding: 0 20px;
  height: 64px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(12px); /* 毛玻璃 */
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  position: sticky;
  top: 0;
  z-index: 100;
  gap: 16px;
}

.search-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  flex: 1;
  max-width: 400px;
}

.search-icon {
  position: absolute;
  left: 12px;
  font-size: 14px;
  opacity: 0.5;
}

.path-hint {
  font-size: 0.8rem;
  color: #666;
}

.search-input {
  width: 100%;
  padding: 10px 35px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 10px;
  background: rgba(0, 0, 0, 0.03);
  font-size: 14px;
  transition: all 0.2s ease;
  outline: none;
}

.search-input:focus {
  background: white;
  border-color: #007aff; /* 经典的蓝色 */
  box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
}

.clear-btn {
  position: absolute;
  right: 10px;
  background: none;
  border: none;
  cursor: pointer;
  opacity: 0.3;
  padding: 5px;
}

/* 分隔线 */
.divider {
  width: 1px;
  height: 24px;
  background: rgba(0, 0, 0, 0.1);
}

.actions {
  display: flex;
  align-items: center;
  gap: 12px; /* 按钮和路径标签之间的间距 */
}

.icon-btn {
  background: transparent;
  border: 1px solid rgba(0, 0, 0, 0.1);
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: #f5f5f5;
}

.icon-btn.active {
  background: #e6f7ff;
  border-color: #1890ff;
  color: #1890ff;
}

.icon-btn .icon {
  font-size: 18px;
}

/* 按钮样式 */
.primary-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 500;
  font-size: 14px;
  cursor: pointer;
  transition:
    transform 0.1s,
    background 0.2s;
}

.primary-btn:hover {
  background: #0063cc;
}

.primary-btn:active {
  transform: scale(0.96);
}

/* 路径标签 */
.path-badge {
  font-size: 12px;
  background: rgba(0, 0, 0, 0.05);
  padding: 4px 10px;
  border-radius: 100px;
  color: #666;
  max-width: 150px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.meme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
}

.meme-card {
  background: white;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s;
}

.meme-card:hover {
  transform: scale(1.05);
}

.meme-card img {
  width: 100%;
  aspect-ratio: 1 / 1;
  object-fit: contain;
  display: block;
}

.empty-state {
  text-align: center;
  color: #999;
  margin-top: 100px;
}
</style>
