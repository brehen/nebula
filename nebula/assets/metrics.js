function setCharts(metrics) {
  let labels = Object.keys(metrics).sort();
  const dockerStartupData = labels.map(
    (label) => metrics[label].docker.avg_startup_time,
  );
  const dockerRuntimeData = labels.map(
    (label) => metrics[label].docker.avg_runtime,
  );
  const dockerTotalRuntimeData = labels.map(
    (label) => metrics[label].docker.avg_total_runtime,
  );
  const wasmStartupData = labels.map(
    (label) => metrics[label].wasm.avg_startup_time,
  );
  const wasmRuntimeData = labels.map(
    (label) => metrics[label].wasm.avg_runtime,
  );
  const wasmTotalRuntimeData = labels.map(
    (label) => metrics[label].wasm.avg_total_runtime,
  );

  labels = labels.map((label) => label.split("").slice(0, 4).join(""));

  const avgStartupCtx = document.getElementById("avgStartupChart");
  const avgRuntimeCtx = document.getElementById("avgRuntimeChart");
  const avgTotalTimeCtx = document.getElementById("avgTotalTimeChart");

  new Chart(avgStartupCtx, {
    type: "bar",
    options: {
      scales: {
        x: {
          stacked: true,
        },
        y: {
          stacked: true,
        },
      },
    },
    data: {
      labels,
      datasets: [
        {
          label: "Wasm",
          data: wasmStartupData,
          borderWidth: 1,
        },
        {
          label: "Docker",
          data: dockerStartupData,
          borderWidth: 1,
        },
      ],
    },
  });

  new Chart(avgRuntimeCtx, {
    type: "bar",
    options: {
      scales: {
        y: {},
      },
    },
    data: {
      labels,
      datasets: [
        {
          label: "Wasm",
          data: wasmRuntimeData,
          borderWidth: 1,
        },
        {
          label: "Docker",
          data: dockerRuntimeData,
          borderWidth: 1,
        },
      ],
    },
  });

  new Chart(avgTotalTimeCtx, {
    type: "bar",
    options: {
      scales: {
        x: {
          stacked: true,
        },
        y: {
          stacked: true,
        },
      },
    },
    data: {
      labels,
      datasets: [
        {
          label: "Wasm",
          data: wasmTotalRuntimeData,
          borderWidth: 1,
        },
        {
          label: "Docker",
          data: dockerTotalRuntimeData,
          borderWidth: 1,
        },
      ],
    },
  });
}
