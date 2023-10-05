// console.log("Doing something");
process.stdin.setEncoding("utf8");

process.stdin.on("data", (data) => {
  const n = parseInt(data, 10);
  const fibNumbers = fibonacci(n);
  process.stdout.write(`[${fibNumbers}]\n`);
  process.exit();
});

const fibonacci = (n) => {
  const sequence = [0, 1];
  for (let i = 2; i < n; i++) {
    sequence[i] = sequence[i - 1] + sequence[i - 2];
  }
  return sequence.slice(0, n);
};
