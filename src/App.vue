<template>
  <div class="flex h-screen w-screen overflow-hidden bg-main-bg select-none">
    <SideBar
      v-model:currentCategoryId="activeCategoryId"
      :collapsed="isCollapsed"
      class="flex-shrink-0 transition-all duration-300 ease-in-out"
      @refresh-memes="triggerRefresh"
    />
    <div class="flex flex-1 flex-col min-w-0">
      <TopBar
        :active-cat-id="activeCategoryId"
        :is-sidebar-collapsed="isCollapsed"
        @import-success="handleImportSuccess"
        @toggle-sidebar="isCollapsed = !isCollapsed"
        @open-settings="showSettings = true"
      />
      <main class="flex-1 overflow-y-auto bg-gray-50/50 relative">
        <transition name="fade" mode="out-in">
          <MainView :category-id="activeCategoryId" :key="refreshKey" />
        </transition>
      </main>
    </div>
  </div>
  <SettingsDialog v-model="showSettings" />
</template>

<script setup>
import { ref } from "vue";
import SideBar from "./components/SideBar.vue";
import TopBar from "./components/TopBar.vue";
import MainView from "./views/MainView.vue";
import SettingsDialog from "./components/SettingsDialog.vue";

const isCollapsed = ref(false);
const activeCategoryId = ref(1);
const refreshKey = ref(0);
const showSettings = ref(false);

const handleImportSuccess = (result) => {
  console.log("导入成功: ", result);
  triggerRefresh();
};

const triggerRefresh = () => {
  refreshKey.value++;
};
</script>

<style scoped>
/* 路由/组件切换的平滑淡入淡出 */
.fade-enter-active,
.fade-leave-active {
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}
.fade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}
.fade-leave-to {
  opacity: 0;
}
</style>
