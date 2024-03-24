module.exports = {
  plugins: [
    require("postcss-preset-env")({
      browsers: "last 2 versions",
    }),
    require("css-declaration-sorter")({
      order: "smacss",
    }),
    require("postcss-prune-var")(),
  ],
};
