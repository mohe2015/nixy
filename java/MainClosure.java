public class MainClosure implements NixLazy {

	public NixValue force() {
		return NixLambda.createFunction(a -> {
			return
					a.add(NixInteger.create(1)).force();
		}).force();
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call());
	}
}
