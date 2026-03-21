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
    class="meme-card group relative aspect-square bg-transparent rounded-xl border-0 cursor-grab active:cursor-grabbing hover:shadow-md transition-all"
  >
    <div
      class="w-full h-full p-0 flex items-center justify-center bg-gray-50/50"
    >
      <img
        :src="convertFileSrc(meme.file_path)"
        class="w-full h-full object-contain p-2"
        loading="lazy"
      />
    </div>

    <div
      class="absolute inset-0 bg-white/40 backdrop-blur-[1px] opacity-0 group-hover:opacity-100 transition-all flex items-center justify-center gap-3"
    >
      <button
        @click.stop.prevent="copyMeme(meme.file_path)"
        title="复制到剪贴板"
        class="no-drag w-12 h-12 bg-white shadow-lg rounded-full flex items-center justify-center text-xl hover:bg-blue-600 hover:text-white transform hover:-translate-y-1 active:scale-90 transition-all"
      >
        📋
      </button>
    </div>

    <div
      v-if="meme.is_gif"
      class="absolute top-2 right-2 px-1.5 py-0.5 bg-black/40 backdrop-blur-md text-white text-[10px] font-bold rounded-md tracking-wider"
    >
      GIF
    </div>
  </div>
</template>
