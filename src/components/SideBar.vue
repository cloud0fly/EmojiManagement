<template>
  <aside
    v-bind="$attrs"
    :class="[
      $attrs.class,
      'h-screen flex flex-col bg-white/80 backdrop-blur-xl border-r border-gray-200/50 transition-all duration-300 ease-in-out',
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
        <div class="flex items-center group mb-1 h-11 w-full">
          <div
            v-if="!collapsed && !cat.isEditing"
            class="drag-handle flex items-center justify-center w-8 h-full cursor-grab active:cursor-grabbing text-gray-300 hover:text-blue-500 opacity-50 group-hover:opacity-100 transition-all shrink-0"
            style="pointer-events: auto"
          >
            <svg
              viewBox="0 0 20 20"
              fill="currentColor"
              class="w-4 h-4 pointer-events-none"
            >
              <path
                d="M7 2a2 2 0 100 4 2 2 0 000-4zM7 8a2 2 0 100 4 2 2 0 000-4zM7 14a2 2 0 100 4 2 2 0 000-4zM13 2a2 2 0 100 4 2 2 0 000-4zM13 8a2 2 0 100 4 2 2 0 000-4zM13 14a2 2 0 100 4 2 2 0 000-4z"
              />
            </svg>
          </div>

          <div
            class="flex-1 relative h-full flex items-center gap-3 px-3 rounded-xl cursor-pointer transition-all duration-200 overflow-hidden"
            :class="getItemClass(currentCategoryId, cat.id)"
            @click="$emit('update:currentCategoryId', cat.id)"
            @mouseenter="onEnter(cat.id)"
            @mouseleave="onLeave"
            @mouseup="handleDrop(cat.id)"
          >
            <div
              v-if="currentCategoryId === cat.id"
              class="absolute left-0 w-1 h-5 bg-white rounded-r-full"
            ></div>

            <span class="text-xl shrink-0">{{
              currentCategoryId === cat.id ? "📂" : "📁"
            }}</span>

            <div v-if="!collapsed" class="flex-1 overflow-hidden">
              <span v-if="!cat.isEditing" class="text-sm truncate block">{{
                cat.name
              }}</span>
              <input
                v-else
                v-model="editName"
                :ref="(el) => (renameInput = el)"
                class="w-full text-sm bg-white/50 border border-blue-400 rounded px-1 outline-none text-gray-800"
                @click.stop
                @blur="handleRename(cat)"
                @keyup.enter="handleRename(cat)"
                @keyup.esc="cat.isEditing = false"
              />
            </div>

            <div
              class="ml-auto flex items-center shrink-0"
              v-if="!collapsed && !cat.isEditing"
            >
              <button
                class="opacity-0 group-hover:opacity-100 transition text-gray-400 hover:text-gray-700"
                @click.stop="toggleMenu(cat.id, $event)"
              >
                ⋯
              </button>
            </div>
          </div>
        </div>
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

  <Teleport to="body">
    <div
      v-if="activeMenuId !== null"
      class="fixed w-28 bg-white border border-gray-200 overflow-hidden rounded-lg shadow-lg z-9999"
      :style="menuStyle"
      @mouseenter="menuHover = true"
      @mouseleave="
        menuHover = false;
        closeMenu();
      "
    >
      <div
        class="px-3 py-2 text-sm hover:bg-gray-100 cursor-pointer"
        @click="renameCategory"
      >
        重命名
      </div>
      <div
        class="px-3 py-2 text-sm text-red-500 hover:bg-red-50 cursor-pointer"
        @click="openDeleteModal"
      >
        删除
      </div>
    </div>
  </Teleport>

  <Teleport to="body">
    <div
      v-if="showConfirmDelete"
      class="fixed inset-0 bg-black/40 backdrop-blur-sm flex items-center justify-center z-10000"
    >
      <div
        class="bg-white rounded-2xl p-6 w-80 shadow-2xl animate-in zoom-in-95 duration-200"
      >
        <h3 class="text-lg font-bold text-gray-800">确认删除分类？</h3>
        <p class="text-sm text-gray-500 mt-2">
          分类内的所有表情包将自动移至
          <span class="font-medium text-blue-600">默认分类</span>。
        </p>
        <div class="flex justify-end gap-3 mt-6">
          <button
            @click="showConfirmDelete = false"
            class="px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 rounded-xl"
          >
            取消
          </button>
          <button
            @click="confirmDelete"
            :disabled="isDeleting"
            class="px-4 py-2 text-sm font-medium bg-red-500 text-white hover:bg-red-600 rounded-xl flex items-center gap-2"
          >
            <span v-if="isDeleting" class="animate-spin text-[10px]">⏳</span>
            {{ isDeleting ? "处理中..." : "确认删除" }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, onMounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import draggable from "vuedraggable";
import { draggingMeme } from "../stores/dragStore";

const props = defineProps(["collapsed", "currentCategoryId"]);
const emit = defineEmits(["update:currentCategoryId", "refresh-memes"]);
defineOptions({ inheritAttrs: false });
 
const menuStyle = ref({});
const menuHover = ref(false);

const currentCat = ref(null);// 当前分类

// 获取所有分类
const categories = ref([]);  // 分类列表
const refreshCat = async () => {
  try {
    const res = await invoke("get_categories");
    categories.value = res.map((cat) => ({
      ...cat,
      isEditing: false,
    }));
    console.log(categories.value);
  } catch (e) {
    console.error("加载分类失败:", e);
  }
};


// 拖入处理
const hoverCatId = ref(null); // 当前分类(拖入用)
const handleDrop = (catId) => {
  if (draggingMeme.value) {
    onDrop(catId);
  } else {
    hoverCatId.value = null;
  }
};
const onDrop = async (catId) => {
  const meme = draggingMeme.value;
  if (!meme) return;

  console.log("拖入分类:", catId, meme);
  const targetMemeId = meme.id;
  draggingMeme.value = null;
  hoverCatId.value = null;

  try {
    await invoke("move_memes_to_category", {
      memeIds: [targetMemeId],
      targetCatId: catId,
    });

    emit("refresh-memes");
  } catch (err) {
    console.error("移动失败:", err);
    emit("refresh-memes");
  }
};
const onEnter = (catId) => {
  if (draggingMeme.value) {
    hoverCatId.value = catId;
  }
};
const onLeave = () => {
  hoverCatId.value = null;
};


// 打开菜单
const activeMenuId = ref(null); // 当前激活的菜单
// const renamingCatId = ref(null);
const toggleMenu = (id, event) => {
  if (activeMenuId.value === id) {
    closeMenu();
    return;
  }

  const rect = event.currentTarget.getBoundingClientRect();

  menuStyle.value = {
    top: rect.bottom + 4 + "px",
    left: rect.right - 120 + "px",
  };

  activeMenuId.value = id;
  currentCat.value = categories.value.find((c) => c.id === id);
};

// 关闭菜单
const closeMenu = () => {
  activeMenuId.value = null;
  currentCat.value = null;
};


// 编辑状态
const editName = ref("");
const enterEditMode = async (cat) => {
  currentCat.value = cat;
  editName.value = cat.name;

  cat.isEditing = true;

  await nextTick();
  renameInput.value?.focus();
  renameInput.value?.select();
};


// 重命名
const renameInput = ref(null);
const renameCategory = async () => {
  if (!currentCat.value) return;

  const renamingCat = currentCat.value;
  closeMenu();
  if (renamingCat) {
    enterEditMode(renamingCat);
  }
};

const handleRename = async (cat) => {
  const trimmedName = editName.value.trim();
  if (!trimmedName || trimmedName === cat.name) {
    cat.isEditing = false;
    return;
  }

  try {
    await invoke("rename_category", {
      catId: cat.id,
      newName: trimmedName,
    });
    cat.name = trimmedName;
    cat.isEditing = false;
  } catch (e) {
    console.error("重命名失败", e);
    cat.isEditing = false;
  }
};


// 删除
const showConfirmDelete = ref(false);
const isDeleting = ref(false);
const deletingCat = ref(null);

const openDeleteModal = () => {
  deletingCat.value = null;

  if (currentCat.value?.id === 1) {
    alert("默认分类不可删除");
    closeMenu();
    return;
  }

  showConfirmDelete.value = true;
  deletingCat.value = currentCat.value;
  closeMenu();
};

const confirmDelete = async () => {
  if (!deletingCat.value || isDeleting.value) return;

  isDeleting.value = true;

  try {
    await invoke("delete_category", { catId: deletingCat.value.id });
    if (props.currentCategoryId === deletingCat.value.id) {
      emit("update:currentCategoryId", 1);
    }
    await refreshCat();
    emit("refresh-memes");
    showConfirmDelete.value = false;
  } catch (e) {
    console.error("删除失败:", e);
  } finally {
    isDeleting.value = false;
  }
};


// 新建
const createCategory = async () => {
  try {
    const newId = await invoke("create_category", {
      name: "新分组",
    });
    await refreshCat();

    const newCat = categories.value.find((c) => c.id === newId);
    if (newCat) {
      await enterEditMode(newCat);
    }
  } catch (err) {
    console.error("新建分组失败:", err);
  }
};


const getItemClass = (categoryId, catId) => {
  const isEditing = categories.value.find((c) => c.id === catId)?.isEditing;
  return [
    "relative group flex items-center gap-3 px-3 py-2.5 rounded-xl cursor-pointer transition-all duration-200",
    categoryId === catId && !isEditing
      ? "bg-blue-600 text-white shadow-md shadow-blue-200"
      : "text-gray-600 hover:bg-gray-100",
    hoverCatId.value === catId ? "ring-2 ring-blue-400 bg-blue-50/50" : "",
    isEditing ? "bg-gray-50 ring-1 ring-blue-300" : "",
  ];
};

onMounted(async () => {
  refreshCat();
  window.addEventListener("click", (e) => {
    if (!menuHover.value) closeMenu();
  });
});
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
.drag-ghost {
  background: #ebf5ff !important;
  border: 2px dashed #60a5fa !important;
}
.dragging-fallback {
  opacity: 0.8;
  background: white !important;
  box-shadow:
    0 10px 25px -5px rgba(0, 0, 0, 0.1),
    0 10px 10px -5px rgba(0, 0, 0, 0.04);
  z-index: 10000 !important; /* 确保层级最高 */
  pointer-events: none !important; /* 镜像不阻挡鼠标事件 */
}

/* 确保拖拽时 body 不会因为滚动条跳动而影响体验 */
body.sortable-dragging {
  overflow: hidden;
}
</style>
