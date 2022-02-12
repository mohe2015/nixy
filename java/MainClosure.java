public class MainClosure implements NixLazy {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		return NixArray.create(java.util.Arrays.asList(NixString.create("""
1"""),NixString.create("""
true"""),NixString.create("""
yes"""))).force().call(false);
	}
}
