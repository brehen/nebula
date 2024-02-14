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

  //labels = labels.map((label) => label.split("").slice(0, 4).join(""));

  const avgStartupCtx = document.getElementById("chart-1");
  const avgRuntimeCtx = document.getElementById("chart-2");
  const avgTotalTimeCtx = document.getElementById("chart-3");

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
          // stacked: true,
        },
        y: {
          // stacked: true,
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

function map_metricsF(metric, module) {
  const { wasm, docker } = metric[module];
  console.log(wasm);
  const waasm = Object.entries(wasm).reduce(
    (acc, [label, stats]) => {
      acc.labels.push(label);
      acc.startup.push(stats.startup[2]);
      acc.runtime.push(stats.runtime[2]);
      acc.total_time.push(stats.total_time[2]);
      return acc;
    },
    {
      labels: [],
      startup: [],
      runtime: [],
      total_time: [],
    },
  );
  const doocker = Object.entries(docker).reduce(
    (acc, [label, stats]) => {
      acc.labels.push(label);
      acc.startup.push(stats.startup[2]);
      acc.runtime.push(stats.runtime[2]);
      acc.total_time.push(stats.total_time[2]);
      return acc;
    },
    {
      labels: [],
      startup: [],
      runtime: [],
      total_time: [],
    },
  );

  const chart1 = document.getElementById("chart-1");
  const chart2 = document.getElementById("chart-2");
  const chart3 = document.getElementById("chart-3");

  new Chart(chart1, {
    type: "line",
    data: {
      labels: waasm.labels,
      datasets: [
        {
          label: "Wasm",
          data: waasm.startup,
          borderWidth: 1,
        },
        // {
        //   label: "Docker",
        //   data: doocker.startup,
        //   borderWidth: 1,
        // },
      ],
    },
  });
  new Chart(chart2, {
    type: "line",
    data: {
      labels: waasm.labels,
      datasets: [
        {
          label: "Wasm",
          data: waasm.runtime,
          borderWidth: 1,
        },
        // {
        //   label: "Docker",
        //   data: doocker.startup,
        //   borderWidth: 1,
        // },
      ],
    },
  });
  new Chart(chart3, {
    type: "line",
    data: {
      labels: waasm.labels,
      datasets: [
        {
          label: "Wasm",
          data: waasm.total_time,
          borderWidth: 1,
        },
        // {
        //   label: "Docker",
        //   data: doocker.startup,
        //   borderWidth: 1,
        // },
      ],
    },
  });

  console.log(waasm, doocker);
}
