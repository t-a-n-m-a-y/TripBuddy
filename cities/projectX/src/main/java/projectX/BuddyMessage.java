package projectX;

public class BuddyMessage {
	
	private String message;
	
	public BuddyMessage() {
    	}

	public void setMessage(String message) {
		this.message = message;
	}
	
	public BuddyMessage(String message_) {
		this.setMessage(message_);
	}

	public String getMessage() {
		return message;
	}
}
