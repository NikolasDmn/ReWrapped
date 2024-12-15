export function create_donut_chart(data) {
  // Parse the data (assumes data is a JSON string passed from Rust)
  const parsedData = JSON.parse(data);

  const series = parsedData.map((item) => item.value);
  const labels = parsedData.map((item) => item.name);
  const colors = parsedData.map((item) => item.color);


  const chartConfig = {
    series: series,
    chart: {
      type: "donut",
      width: "100%",
      height: "100%",
      toolbar: {
        show: false,
      },
      axisBorder: {
        show: false,
      },
      axisTicks: {
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
      formatter: function (val, opts) {
        return `${opts.w.globals.labels[opts.seriesIndex]}: ${opts.w.globals.series[opts.seriesIndex]} %`;

      },
    },
    stroke: {
      show: true,
      width: 2,
      colors: ["#121212"],
    }, legend: {
      show: false, // Hides the legend
    }, xaxis: {
      labels: {
        show: false,
      },
      axisBorder: {
        show: false,
      },
      axisTicks: {
        show: false,
      },
    },
    yaxis: {
      labels: {
        show: false,
      },
      axisBorder: {
        show: false,
      },
    },
    grid: {
      show: false,
    }

  };

  // Ensure the chart container exists
  const chartContainer = document.getElementById("pie-chart");
  if (!chartContainer) {
    console.error("Error: <div id='pie-chart'> not found in the DOM.");
    return;
  }

  // Clear any existing chart (if necessary)
  while (chartContainer.firstChild) {
    chartContainer.removeChild(chartContainer.firstChild);
  }

  // Render the chart in the container
  const chart = new ApexCharts(chartContainer, chartConfig);
  chart.render();
}


