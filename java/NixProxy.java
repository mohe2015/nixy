public class NixProxy implements NixLazy {

	NixLazy proxy;

	@Override
	public NixValue force() {
		return proxy.force();
	}
}
