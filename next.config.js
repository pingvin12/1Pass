/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    domains: ["lh3.googleusercontent.com", "avatars.githubusercontent.com"],
  },
  experimental: {
    appDir: true,
  },
  port: 4200,
  distDir: "./out",
  output: "export",
};

module.exports = nextConfig;
