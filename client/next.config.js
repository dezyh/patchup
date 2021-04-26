module.exports = {
  async rewrites() {
    return process.env.CLIENT_ENV === 'development'
      ? [{ source: '/api/:path*', destination: 'http://patchup-server:8080/api/:path*' }]
      : []
  }
}
