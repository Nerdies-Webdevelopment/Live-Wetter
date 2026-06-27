(() => {
  if (!("serviceWorker" in navigator)) {
    return;
  }

  window.addEventListener("load", () => {
    const scope = new URL(".", document.baseURI);
    const worker = new URL("sw.js", scope);

    navigator.serviceWorker.register(worker, { scope: scope.pathname }).catch(() => {
      // Die App bleibt ohne Service Worker normal nutzbar.
    });
  });
})();
