public class MainClosure implements NixObject {

	public NixObject call(NixObject arg) {
		NixObject.ensureLazy(arg);
		return NixObject.createFunction(a -> {
			return
					a.add(NixInteger.create(1)).force();
		}).force();
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure().force());
	}
}
