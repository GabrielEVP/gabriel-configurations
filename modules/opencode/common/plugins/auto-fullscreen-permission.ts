import type { TuiPluginModule } from "@opencode-ai/plugin/tui";

const autoFullscreenPermission: TuiPluginModule = {
  id: "auto-fullscreen-permission",
  tui: async (api) => {
    let isFullscreen = false;
    const processed = new Set<string>();

    api.event.on("permission.asked", (event) => {
      const permId = event.properties.id;
      if (processed.has(permId)) return;
      processed.add(permId);
      isFullscreen = false;

      setTimeout(() => {
        if (!isFullscreen) {
          api.keymap.dispatchCommand("permission.prompt.fullscreen");
          isFullscreen = true;
        }
      }, 50);
    });

    api.event.on("permission.replied", () => {
      isFullscreen = false;
    });
  },
};

export default autoFullscreenPermission;
