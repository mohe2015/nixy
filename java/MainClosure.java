public class MainClosure implements NixLazy {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		return NixInteger.create(1).add(() -> {
			// head
			NixLazy a = NixInteger.create(5);
			NixLazy b = NixInteger.create(7);

			// body
			return a.add(b).force();
		}).force();
	}
}
