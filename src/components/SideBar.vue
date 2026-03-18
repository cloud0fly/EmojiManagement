<template>
  <aside 
    :class="[
      'h-screen flex flex-col bg-white/80 backdrop-blur-xl border-r border-gray-200/50 transition-all duration-300 ease-in-out select-none',
      collapsed ? 'w-18' : 'w-64'
    ]"
  >
    <div class="h-14 flex items-center px-4 border-b border-gray-100 font-semibold text-gray-700 overflow-hidden">
      <span v-if="!collapsed" class="truncate animate-in fade-in slide-in-from-left-2">📂 我的表情仓库</span>
      <span v-else class="mx-auto text-xl">📦</span>
    </div>
    
    <nav class="flex-1 mt-3 px-2 space-y-1 overflow-y-auto custom-scrollbar">
      <div 
        v-for="cat in categories" :key="cat.id"
        @click="$emit('update:currentCategoryId', cat.id)"
        @dragover.prevent="dragOverItem = cat.id"
        @dragleave="dragOverItem = null"
        @drop="handleDrop($event, cat.id)"
        :class="[
          'relative group flex items-center gap-3 px-3 py-2.5 rounded-xl cursor-pointer transition-all duration-200',
          // 选中状态：蓝色背景
          currentCategoryId === cat.id 
            ? 'bg-blue-600 text-white shadow-md shadow-blue-200' 
            : 'text-gray-600 hover:bg-gray-100',
          // 拖拽悬停状态：高亮反馈 (Tailwind v4 语法)
          dragOverItem === cat.id && currentCategoryId !== cat.id
            ? 'ring-2 ring-blue-400 ring-offset-2 bg-blue-50/50 scale-[1.02] translate-x-1'
            : ''
        ]"
      >
        <div 
          v-if="currentCategoryId === cat.id" 
          class="absolute left-0 w-1 h-5 bg-white rounded-r-full"
        ></div>

        <span class="text-xl filter drop-shadow-sm group-hover:scale-110 transition-transform">
          {{ currentCategoryId === cat.id ? '📂' : '📁' }}
        </span>

        <div v-if="!collapsed" class="flex-1 flex items-center justify-between overflow-hidden">
          <span class="text-sm font-medium truncate">{{ cat.name }}</span>
          <span v-if="currentCategoryId !== cat.id" class="opacity-0 group-hover:opacity-40 text-[10px]">
            →
          </span>
        </div>
      </div>
    </nav>

    <div class="p-4 border-t border-gray-100 space-y-2">
      <button v-if="!collapsed" class="w-full py-2 text-xs font-medium text-blue-600 hover:bg-blue-50 rounded-lg transition-colors border border-dashed border-blue-200">
        + 新建分组
      </button>
    </div>
  </aside>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps(['collapsed', 'currentCategoryId']);
const emit = defineEmits(['update:currentCategoryId', 'refresh-memes']);

const categories = ref([]);
const dragOverItem = ref(null);

onMounted(async () => {
  try {
    categories.value = await invoke('get_categories');
  } catch (e) {
    console.error("加载分类失败:", e);
  }
});

const handleDrop = async (event, catId) => {
  event.preventDefault();
  dragOverItem.value = null;

  const memeId = event.dataTransfer.getData('memeId');
  if (memeId) {
    try {
      // 这里的参数名要和 Rust 端的 move_meme_to_category 对齐
      await invoke('move_meme_to_category', { 
        memeId: parseInt(memeId), 
        targetCatId: catId 
      });
      // 触发父组件刷新 MainView
      emit('refresh-memes');
    } catch (err) {
      console.error('移动失败:', err);
    }
  }
};
</script>

<style scoped>
/* 自定义滚动条，让 UI 更精致 */
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: transparent;
  border-radius: 10px;
}
.custom-scrollbar:hover::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
}
</style>