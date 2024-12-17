export function create_bar_chart(data) {
  // Parse the data (assumes data is a JSON string passed from Rust)
  const parsedData = JSON.parse(data);
  const series = parsedData.map((item) => item.value);
  const categories = parsedData.map((item) => item.name);
  const colors = parsedData.map((item) => item.color);
  const chartConfig = {
    series: [
      {
        name: "Value",
        data: series,
      },
    ],
    chart: {
      type: "bar",
      width: "100%",
      height: "100%",
      toolbar: {
        show: false,
      },
    },
    xaxis: {
      categories: categories,
      labels: {
        show: true,
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
      axisTicks: {
        show: false,
      },
    },
    title: {
      show: false,
    },
    dataLabels: {
      position: "top",
    },
    colors: colors,
    stroke: {
      show: true,
      width: 2,
      colors: ["#121212"],
    },
    legend: {
      show: false,
    },
    grid: {
      show: false,
    },
    plotOptions: {
      bar: {
        borderRadius: 6,
        distributed: true,
      },
    },
  };
  const chartContainer = document.getElementById("bar-chart");
  if (!chartContainer) {
    console.error("Error: <div id='bar-chart'> not found in the DOM.");
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
