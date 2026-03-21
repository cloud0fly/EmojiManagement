<script setup>
import { ref, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { loadSettings, updateSetting } from "../stores/settingStore";
import { getCurrentWindow } from "@tauri-apps/api/window";
const appWindow = getCurrentWindow();

const modelValue = defineModel();
const settings = ref(null);

watch(modelValue, async (isOpen) => {
  if (isOpen) {
    const data = await loadSettings();
    settings.value = JSON.parse(JSON.stringify(data));
  }
});

watch(
  () => settings.value?.alwaysOnTop,
  async (newValue) => {
    if (newValue !== undefined) {
      await appWindow.setAlwaysOnTop(newValue);
    }
  }
);

const selectPath = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择表情包存放目录",
  });
  if (selected) {
    settings.value.memePath = selected;
  }
};

const handleSave = async () => {
  if (!settings.value) return;
  const oldSettings = await loadSettings();
  console.log("new ", settings.value);
  console.log("old ", oldSettings.memePath);

  if (settings.value.memePath !== oldSettings.memePath) {
    try {
      await invoke("migrate_memes_data", { newPath: settings.value.memePath });
      console.log("文件迁移成功");
    } catch (err) {
      alert(`迁移失败: ${err}`);
      return;
    }
  }

  const updatePromises = Object.entries(settings.value).map(([key, value]) => {
    return updateSetting(key, value);
  });

  await Promise.all(updatePromises);
  modelValue.value = false;
};

const openLogFolder = async () => {
  try {
    await invoke("open_log_dir");
  } catch (err) {
    console.error("无法打开日志目录:", err);
  }
};
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-300 ease-out"
      enter-from-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition duration-200 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <div
        v-if="modelValue"
        class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
        @click.self="modelValue = false"
      >
        <div
          class="bg-white dark:bg-zinc-900 w-full max-w-md rounded-4xl shadow-2xl border border-zinc-200 dark:border-white/10 overflow-hidden"
        >
          <div
            class="px-8 py-6 flex justify-between items-center border-b dark:border-white/5"
          >
            <div>
              <h2 class="text-xl font-bold text-zinc-800 dark:text-zinc-100">
                设置
              </h2>
              <!-- <p class="text-xs text-zinc-400 mt-1">配置您的表情管理中心</p> -->
            </div>
            <button
              @click="modelValue = false"
              class="p-2 hover:bg-zinc-100 dark:hover:bg-white/5 rounded-full text-zinc-400 transition-colors"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="w-5 h-5"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="12"></line>
              </svg>
            </button>
          </div>

          <div class="p-8 space-y-8 max-h-[65vh] overflow-y-auto">
            <div v-if="settings" class="space-y-8">
              <div class="space-y-3">
                <label
                  class="text-[11px] font-black text-blue-500 uppercase tracking-[0.2em]"
                  >数据存储</label
                >
                <div
                  class="flex gap-2 p-2 bg-zinc-50 dark:bg-black/20 rounded-2xl border border-zinc-100 dark:border-white/5 focus-within:ring-2 ring-blue-500/20 transition-all"
                >
                  <input
                    v-model="settings.memePath"
                    readonly
                    placeholder="未配置路径"
                    class="flex-1 bg-transparent px-3 py-2 text-sm text-zinc-600 dark:text-zinc-300 outline-none truncate"
                  />
                  <button
                    @click="selectPath"
                    class="px-5 py-2 bg-white dark:bg-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-700 text-zinc-700 dark:text-zinc-200 rounded-xl text-xs font-bold shadow-sm border border-zinc-200 dark:border-white/10 transition-all active:scale-95"
                  >
                    浏览
                  </button>
                </div>
              </div>

              <div class="space-y-3">
                <label
                  class="text-[11px] font-black text-blue-500 uppercase tracking-[0.2em]"
                  >交互行为</label
                >
                <div
                  class="bg-zinc-50/50 dark:bg-white/5 rounded-3xl px-6 py-2 divide-y divide-zinc-100 dark:divide-white/5"
                >
                  <div
                    v-for="(label, key) in {
                      alwaysOnTop: '保持窗口置顶',
                    }"
                    :key="key"
                    class="flex items-center justify-between py-4"
                  >
                    <span
                      class="text-sm font-medium text-zinc-700 dark:text-zinc-300"
                      >{{ label }}</span
                    >
                    <label
                      class="relative inline-flex items-center cursor-pointer"
                    >
                      <input
                        type="checkbox"
                        v-model="settings[key]"
                        class="sr-only peer"
                      />
                      <div
                        class="w-10 h-6 bg-zinc-300 dark:bg-zinc-700 peer-focus:outline-none rounded-full peer peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-1 after:inset-s-1 after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-4"
                      ></div>
                    </label>
                  </div>
                </div>
              </div>

              <div class="space-y-3"></div>
              <label
                class="text-[11px] font-black text-blue-500 uppercase tracking-[0.2em]"
                >调试用</label
              >
              <button
                @click="openLogFolder"
                style="padding: 6px 12px; font-size: 13px; cursor: pointer"
              >
                📂 打开日志文件夹
              </button>
            </div>
          </div>

          <div
            class="px-8 py-6 bg-zinc-50/50 dark:bg-black/20 flex justify-end gap-3 border-t dark:border-white/5"
          >
            <button
              @click="modelValue = false"
              class="px-6 py-2.5 text-sm font-bold text-zinc-400 hover:text-zinc-600 transition-colors"
            >
              取消
            </button>
            <button
              @click="handleSave"
              class="px-8 py-2.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white text-sm font-bold rounded-2xl shadow-xl shadow-blue-500/20 transition-all active:scale-95"
            >
              保存更改
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
/* 移除所有使用 @apply 的自定义类，这里只放原生的 CSS 或滚动条美化 */
::-webkit-scrollbar {
  width: 5px;
}
</style>
