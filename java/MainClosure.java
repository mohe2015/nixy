public class MainClosure implements NixLazy {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		return NixLambda.createFunction((a) -> a.add(NixInteger.create(1)).force()).force();
	}
}
