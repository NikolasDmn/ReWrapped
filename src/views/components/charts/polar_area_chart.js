
export function create_polar_area_chart(data) {
  // Parse the data (assumes data is a JSON string passed from Rust)
  const parsedData = JSON.parse(data);

  const series = parsedData.map((item) => item.value);
  const labels = parsedData.map((item) => item.name);
  const colors = parsedData.map((item) => item.color);
  const chartConfig = {
    series: series,
    chart: {
      type: "polarArea",
      width: "100%",
      height: "100%",
      toolbar: {
        show: false,
      },
    },
    labels: labels,
    colors: colors,
    title: {
      show: false,
    },
    dataLabels: {
      enabled: true,
      style: {
        fontSize: "12px",
        fontWeight: "bold",
        colors: ["#121212"],
      },
      background: {
        enabled: false,
        foreColor: "#121212",
        padding: 4,
        borderRadius: 2,
      },
      dropShadow: {
        enabled: false,
      },
      formatter: function (val, opts) {
        return `${opts.w.globals.labels[opts.seriesIndex]}: ${opts.w.globals.series[opts.seriesIndex]} %`;
      },
    },
    stroke: {
      show: false,
    },
    legend: {
      show: false,
    },
    grid: {
      show: false,
    },
    plotOptions: {
      polarArea: {
        rings: {
          strokeWidth: 0,
        },
        spokes: {
          strokeWidth: 0,
        },
      },
      pie: {
        borderRadius: 6,
      },
    },
  };

  const chartContainer = document.getElementById("polar-area-chart");
  if (!chartContainer) {
    console.error("Error: <div id='pie-chart'> not found in the DOM.");
    return;
  }

  //Clear any existing chart (if necessary)
  while (chartContainer.firstChild) {
    chartContainer.removeChild(chartContainer.firstChild);
  }

  // Render the chart in the container
  const chart = new ApexCharts(chartContainer, chartConfig);
  chart.render();

}



