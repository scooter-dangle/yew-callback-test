Object.const_set(:ROOT, '.') unless Object.const_defined?(:ROOT)

def read(path)
  [IO.binread("#{ROOT}/#{path}")]
end

CONTENT_TYPES = {
  html: 'text/html',
  js:   'application/javascript',
  wasm: 'application/wasm',
}
CONTENT_TYPES.default = 'text/html'

def headers(content_type: :html)
  {
    'Content-Type' => CONTENT_TYPES[content_type],
  }
end

ROUTES = ->(path) do
  path = '/index.html' if path == ?/
  path = ".#{path}"
  content_type = path.split(?.).last.to_sym
  if File.exist?(path)
    [200, headers(content_type: content_type), read(path)]
  else
    [404, headers, ['Entity not found']]
  end
end

run ->(env) { ROUTES[Rack::Request.new(env).path_info] }
