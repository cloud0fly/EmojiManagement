<template>
  <header
    class="h-14 border-b border-gray-100 bg-white/80 backdrop-blur-md flex items-center justify-between px-4"
  >
    <div class="flex items-center gap-4">
      <button
        @click="$emit('toggle-sidebar')"
        class="p-2 hover:bg-gray-100 rounded-lg transition-colors text-gray-500 hover:text-blue-600"
      >
        <svg
          v-if="isSidebarCollapsed"
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="3" y1="12" x2="21" y2="12"></line>
          <line x1="3" y1="6" x2="21" y2="6"></line>
          <line x1="3" y1="18" x2="21" y2="18"></line>
        </svg>
        <svg
          v-else
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="9" y1="3" x2="9" y2="21"></line>
        </svg>
      </button>

      <h1 class="font-bold text-gray-700">我的表情包</h1>
    </div>

    <div class="flex items-center gap-2">
      <button
        @click="emit('open-settings')"
        class="p-2 hover:bg-gray-100 rounded-lg transition-colors text-gray-500 hover:text-blue-600"
        title="设置"
      >
        ⚙️设置
      </button>

      <button
        @click="handleImport"
        class="px-4 py-2 bg-blue-600 text-white rounded-xl text-sm font-medium hover:bg-blue-700 transition-all"
      >
        批量导入
      </button>
    </div>
  </header>
</template>

<script setup>
//TopBar.vue
import { onMounted, onUnmounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps(["isSidebarCollapsed", "activeCatId"]);
const emit = defineEmits(["toggle-sidebar", "import-success", "open-settings"]);

const handleImport = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择要读入的表情包文件夹",
  });

  if (selected) {
    try {
      const result = await invoke("import_image_from_folder", {
        folderPath: selected,
        targetCatId: props.activeCatId || 1, // 默认导入到第一个分组
      });

      alert(`导入完成！共发现 ${result.total} 张，新增 ${result.new} 张。`);
      emit("import-success");
    } catch (err) {
      console.error("导入失败:", err);
    }
  }
};
</script>
