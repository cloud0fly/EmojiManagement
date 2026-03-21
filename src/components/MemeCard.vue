<script setup>
// MemeCard.vue
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

const props = defineProps(["meme"]);
const emit = defineEmits(["copy-success"]);

const copyMeme = async (path) => {
  try {
    await invoke("copy_image", { path });
    emit("copy-success");
  } catch (err) {
    console.error("复制失败:", err);
  }
};
</script>

<template>
  <div
    class="meme-card group relative aspect-square bg-gray-50/50 rounded-xl overflow-hidden cursor-grab active:cursor-grabbing hover:shadow-md transition-all border border-transparent hover:border-gray-200"
  >
    <div class="w-full h-full flex items-center justify-center p-2">
      <img
        :src="convertFileSrc(meme.file_path)"
        :alt="meme.name || 'meme'"
        class="max-w-full max-h-full object-contain"
        loading="lazy"
      />
    </div>

    <div
      class="absolute inset-0 bg-white/40 backdrop-blur-[2px] opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"
    >
      <button
        @click.stop.prevent="copyMeme(meme.file_path)"
        title="复制到剪贴板"
        class="w-12 h-12 bg-white shadow-xl rounded-full flex items-center justify-center text-xl hover:bg-blue-600 hover:text-white transform hover:-translate-y-1 active:scale-95 transition-all"
      >
        📋
      </button>
    </div>

    <div
      v-if="meme.is_gif"
      class="absolute top-2 right-2 px-1.5 py-0.5 bg-black/50 backdrop-blur-md text-white text-[10px] font-bold rounded shadow-sm"
    >
      GIF
    </div>
  </div>
</template>
