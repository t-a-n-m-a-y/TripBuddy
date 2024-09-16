package projectX;

import javax.ws.rs.DefaultValue;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.client.Client;
import javax.ws.rs.client.ClientBuilder;
import javax.ws.rs.client.WebTarget;
import javax.ws.rs.core.MediaType;
import java.net.URI;
import javax.ws.rs.core.Response;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import com.fasterxml.jackson.databind.ObjectMapper;

@Path("web")
public class TripBuddy {
    
	@GET
	@Produces(MediaType.TEXT_HTML)
	public String getBuddyForm(@DefaultValue("") @QueryParam("inParam1") String city1, @DefaultValue("") @QueryParam("inParam2") String city2) {

		if("".equals(city1) && "".equals(city2)) {
			// Initial case when no input parameter is passed. Show web form.
			String htmlCode = "<!DOCTYPE html> <html lang=\"en\"> <head>"
					+ "<meta charset=\"UTF-8\">" 
					+ "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
					+ "<title>TripBuddy - G6</title> <link rel=\"stylesheet\"> </head>" 
					+ "<body style=\"background-color: #a5affa;\">"
					+ "<div class=\"container\" "
					+ "style=\"background-color: #f7a265; border-radius: 10px; box-shadow: 0 0 10px rgba(0, 0, 0, 0.1); padding:30px; max-width:600px; margin:20px auto;\">" 
					+ "<h1 style=\"text-align: center; color: #000;\">Welcome to TripBuddy</h1>"
					+ "<h3 style=\"text-align: center; color: #666\">by Group 6</h3>"
					+ "<form action=\"web\" method=\"get\">Enter First City <input type=\"text\" name=\"inParam1\"><br>" 
					+ "Enter Second City <input type=\"text\" name=\"inParam2\"><br>" 
					+ "<input type=\"submit\" value=\"Submit\"></form>" 
					+ "</body></html>";

			return htmlCode;


		} else { 
			System.out.println("* Attempting to build client");
			Client c = ClientBuilder.newClient();
			System.out.println("* Client built as --> " + c);
			String restUri = TripBuddyLauncher.restBaseUri;
			System.out.println("* Restbased uri --> " + restUri);            
			WebTarget target = c.target(restUri);
			System.out.println("* WebTarget as --> " + target);
			WebTarget check = target.path("trip").queryParam("msg1", city1).queryParam("msg2", city2);;
			System.out.println("* checking WebTarget with params --> " + check);
			URI uri = check.getUri();
			System.out.println("* RestService url with params --> " + uri);

			try {
				URL url = new URL(uri.toString());
				BuddyMessage responseMsg = null;
				HttpURLConnection connection = (HttpURLConnection) url.openConnection();
				connection.setRequestMethod("GET");

				int responseCode = connection.getResponseCode();
				if (responseCode == HttpURLConnection.HTTP_OK) {
					BufferedReader in = new BufferedReader(new InputStreamReader(connection.getInputStream()));
					String inputLine;
					StringBuffer response = new StringBuffer();

					while ((inputLine = in.readLine()) != null) {
						response.append(inputLine);
					}
					in.close();
					ObjectMapper objectMapper = new ObjectMapper();
					responseMsg = objectMapper.readValue(response.toString(), BuddyMessage.class);	
					System.out.println("Response from server:");
					System.out.println(responseMsg.getMessage());
				} 
				else {
					System.out.println("GET request failed with response code: " + responseCode);
				}
				String htmlResponse = "<!DOCTYPE html> <html lang=\"en\"> <head>"
							+ "<meta charset=\"UTF-8\">" 
							+ "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
							+ "<title>TripBuddy - G6</title> <link rel=\"stylesheet\"> </head>" 
							+ "<body style=\"background-color: #a5affa;\">"
							+ "<div class=\"container\" style=\"background-color: #f7a265; border-radius: 10px; box-shadow: 0 0 10px rgba(0, 0, 0, 0.1); padding: 30px; max-width: 600px; margin: 20px auto;\">" 
							+ "<h1 style=\"text-align: center; color: #111;\">Welcome to TripBuddy</h1>"
							+ "<h3 style=\"text-align: center; color: #666\">by Group 6</h3>"
							+ "<em>" + responseMsg.getMessage() + "</em>"
							+ "</div></body></html>";
				return htmlResponse;
			} catch (IOException e) {
				e.printStackTrace();
				return "<html><body><h2>Welcome to TripBuddy - Group 6</h2>Remote server said <em>" + "error" + "</em></body></html>";
			}

		}
	}
}    
