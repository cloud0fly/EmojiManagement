<template>
  <div class="main-view p-6 h-full overflow-y-auto custom-scrollbar">
    <div
      v-if="memes.length === 0"
      class="flex flex-col items-center justify-center py-32 border-2 border-dashed border-gray-100 rounded-4xl bg-white/30 backdrop-blur-md shadow-sm transition-all"
    >
      <div
        class="text-7xl mb-6 filter drop-shadow-xl grayscale hover:grayscale-0 transition-all duration-500 cursor-default"
      >
        🖼️
      </div>
      <p class="text-zinc-500 font-bold text-lg">当前分组空空如也</p>
      <p class="text-zinc-400 text-sm mt-2 opacity-80">
        点击上方“导入”或直接拖入文件
      </p>
    </div>

    <draggable
      v-model="memes"
      item-key="id"
      class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6"
      ghost-class="ghost-item"
      :group="{ name: 'memes', pull: 'clone', put: false }"
      @start="onDragStart"
      @end="handleSortEnd"
      :force-fallback="true"
      :filter="'.no-drag'"
      :prevent-on-filter="false"
    >
      <template #item="{ element }">
        <div
          class="w-30 aspect-square bg-white rounded-2xl shadow-sm cursor-grab"
        >
          <MemeCard :meme="element" @copy-success="handleCopySuccess" />
          <div
            class="absolute inset-0 rounded-2xl ring-2 ring-blue-500/0 group-hover:ring-blue-500/20 transition-all pointer-events-none"
          ></div>
        </div>
      </template>
    </draggable>

    <transition name="fade">
      <div
        v-if="toast.show"
        class="fixed bottom-6 left-1/2 -translate-x-1/2 bg-black/80 text-white px-5 py-2.5 rounded-xl shadow-lg backdrop-blur-md text-sm font-medium animate-fade-in"
      >
        {{ toast.message }}
      </div>
    </transition>
  </div>
</template>

<script setup>
// MainView.vue
import { ref, onMounted, watch } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import MemeCard from "../components/MemeCard.vue";
import draggable from "vuedraggable";
import { message } from "@tauri-apps/plugin-dialog";
import { draggingMeme } from "../stores/dragStore";

const props = defineProps(["categoryId"]);
const memes = ref([]);
const toast = ref({
  show: false,
  message: "",
});

const onDragStart = async (evt) => {
  draggingMeme.value = evt.item.__draggable_context.element;
};

const handleSortEnd = async (evt) => {
  draggingMeme.value = null;

  if (evt.to === evt.from) {
    console.log("正在保存排序");
    const sortedIds = memes.value.map((m) => m.id);

    try {
      await invoke("update_memes_order", { ids: sortedIds });
    } catch (err) {
      console.error("更新排序失败:", err);
      loadMemes();
    }
  }
};

let timer = null;
const handleCopySuccess = () => {
  toast.value.message = "已复制到剪贴板";
  toast.value.show = true;

  if (timer) clearTimeout(timer);
  setTimeout(() => {
    toast.value.show = false;
  }, 1500);
};

// 加载数据
const loadMemes = async () => {
  try {
    memes.value = await invoke("get_memes_by_category", {
      catId: props.categoryId || 1,
    });
  } catch (err) {
    console.error("加载失败:", err);
  }
};

defineExpose({ loadMemes });

// 监听分组切换
watch(() => props.categoryId, loadMemes);
onMounted(loadMemes);
</script>

<style scoped>
.ghost-item {
  opacity: 0.2;
  border: 2px dashed #3b82f6 !important;
  background: #eff6ff !important;
  box-shadow: none !important;
  transform: scale(0.95);
}
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 10px;
}
.custom-scrollbar:hover::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
}

.main-view {
  animation: slideUp 0.4s ease-out;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
