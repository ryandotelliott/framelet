import React, { useState, useEffect } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

const TitleBar: React.FC = () => {
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    appWindow.isMaximized().then(setIsMaximized);

    const unlisten = appWindow.listen('tauri://resize', async () => {
      setIsMaximized(await appWindow.isMaximized());
    });

    return () => {
      unlisten.then((off: () => void) => {
        off();
      });
    };
  }, []);

  const handleMinimize = async () => await appWindow.minimize();
  const handleMaximize = async () => await appWindow.toggleMaximize();
  const handleClose = async () => await appWindow.close();

  return (
    <div
      data-tauri-drag-region
      className="h-8 flex items-center justify-between bg-gray-100 dark:bg-gray-800 shadow select-none"
    >
      <div className="px-2 text-sm font-medium truncate">Framelet</div>
      <div className="flex">
        <button
          onClick={handleMinimize}
          className="w-8 h-8 flex items-center justify-center hover:bg-gray-200 dark:hover:bg-gray-700"
        >
          ─
        </button>
        <button
          onClick={handleMaximize}
          className="w-8 h-8 flex items-center justify-center hover:bg-gray-200 dark:hover:bg-gray-700"
        >
          {isMaximized ? '🗗' : '🗖'}
        </button>
        <button
          onClick={handleClose}
          className="w-8 h-8 flex items-center justify-center hover:bg-red-500 hover:text-white"
        >
          ×
        </button>
      </div>
    </div>
  );
};

export default TitleBar;
