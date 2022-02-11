public class MainClosure implements NixObject {

	public NixObject call(NixObject arg) {
		return new IntegerAdditionClosure().call(new NixInteger(1)).call(arg);
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure().call(new NixInteger(2)));
	}
}
