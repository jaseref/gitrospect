const tailwindcss = require('tailwindcss'); // eslint-disable-line @typescript-eslint/no-var-requires
const autoprefixer = require('autoprefixer'); // eslint-disable-line @typescript-eslint/no-var-requires

const config = {
    plugins: [
        //Some plugins, like tailwindcss/nesting, need to run before Tailwind,
        tailwindcss(),
        //But others, like autoprefixer, need to run after,
        autoprefixer
    ]
};

module.exports = config;
