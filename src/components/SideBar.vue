<template>
  <aside
    :class="[
      'h-screen flex flex-col bg-white/80 backdrop-blur-xl border-r border-gray-200/50 transition-all duration-300 ease-in-out select-none',
      collapsed ? 'w-18' : 'w-64',
    ]"
  >
    <div
      class="h-14 flex items-center px-4 border-b border-gray-100 font-semibold text-gray-700 overflow-hidden"
    >
      <span
        v-if="!collapsed"
        class="truncate animate-in fade-in slide-in-from-left-2"
        >📂 我的表情仓库</span
      >
      <span v-else class="mx-auto text-xl">📦</span>
    </div>

    <nav class="flex-1 mt-3 px-2 space-y-1 overflow-y-auto custom-scrollbar">
      <div v-for="cat in categories" :key="cat.id">
        <draggable
          :list="[]"
          :group="{ name: 'memes', put: true, pull: false }"
          item-key="id"
          ghost-class="hidden-ghost"
          @add="(evt) => handleMemeAddedToCategory(evt, cat.id)"
        >
          <template #item>
            <div></div>
          </template>
          <template #header>
            <div
              @click="$emit('update:currentCategoryId', cat.id)"
              :class="[
                'relative group flex items-center gap-3 px-3 py-2.5 rounded-xl cursor-pointer transition-all duration-200',
                currentCategoryId === cat.id
                  ? 'bg-blue-600 text-white shadow-md shadow-blue-200'
                  : 'text-gray-600 hover:bg-gray-100',
              ]"
            >
              <div
                v-if="currentCategoryId === cat.id"
                class="absolute left-0 w-1 h-5 bg-white rounded-r-full"
              ></div>

              <span
                class="text-xl filter drop-shadow-sm group-hover:scale-110 transition-transform"
              >
                {{ currentCategoryId === cat.id ? "📂" : "📁" }}
              </span>

              <div
                v-if="!collapsed"
                class="flex-1 flex items-center justify-between overflow-hidden"
              >
                <span class="text-sm font-medium truncate">{{ cat.name }}</span>
              </div>
            </div>
          </template>
        </draggable>
      </div>
    </nav>

    <div class="p-4 border-t border-gray-100 space-y-2">
      <button
        v-if="!collapsed"
        class="w-full py-2 text-xs font-medium text-blue-600 hover:bg-blue-50 rounded-lg transition-colors border border-dashed border-blue-200"
        @click="createCategory"
      >
        + 新建分组
      </button>
    </div>
  </aside>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import draggable from "vuedraggable";

const props = defineProps(["collapsed", "currentCategoryId"]);
const emit = defineEmits(["update:currentCategoryId", "refresh-memes"]);

const categories = ref([]);

const refreshCat = async () => {
  try {
    const res = await invoke("get_categories");

    categories.value = res.map((cat) => ({
      ...cat,
      _dropZone: [],
    }));
  } catch (e) {
    console.error("加载分类失败:", e);
  }
};

onMounted(async () => {
  refreshCat();
});

const handleMemeAddedToCategory = async (evt, catId) => {
  const memeData =
    evt.item?._underlying_vm_ || evt.item?.__draggable_context?.element;

  if (!memeData || !memeData.id) return;

  console.log(`拖入分类: meme=${memeData.id}, target=${catId}`);

  try {
    await invoke("move_memes_to_category", {
      memeIds: [memeData.id],
      targetCatId: catId,
    });

    emit("refresh-memes");
  } catch (err) {
    console.error("移动失败:", err);
    emit("refresh-memes");
  }
};

const createCategory = async () => {
  try {
    await invoke("create_category", {
      name: "未命名分组",
    });

    refreshCat();
  } catch (err) {
    console.error("新建分组并移动失败:", err);
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
