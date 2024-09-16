package projectX;

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.DefaultValue;
import javax.ws.rs.QueryParam;
import javax.ws.rs.core.MediaType;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.google.gson.JsonIOException;
import com.google.gson.JsonParseException;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;

@Path("trip")
public class TripService {
	
	private String extractVal(JsonObject json) {
		// Check if the value exists in the first possible path
		if (json.has("data") && json.getAsJsonObject("data").has("Typeahead_autocomplete")
			&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").has("results")
			&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").size() > 0
			&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(0).getAsJsonObject().has("detailsV2")
			&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(0).getAsJsonObject().getAsJsonObject("detailsV2").has("names")
			&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(0).getAsJsonObject().getAsJsonObject("detailsV2").getAsJsonObject("names").has("name")) {
			return json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(0).getAsJsonObject().getAsJsonObject("detailsV2").getAsJsonObject("names").get("name").getAsString();
		} 
		// Check if the value exists in the second possible path
		else if (json.has("data") && json.getAsJsonObject("data").has("Typeahead_autocomplete")
		&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").has("results")
		&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").size() > 3
		&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(3).getAsJsonObject().has("detailsV2")
		&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(3).getAsJsonObject().getAsJsonObject("detailsV2").has("names")
		&& json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(3).getAsJsonObject().getAsJsonObject("detailsV2").getAsJsonObject("names").has("name")) {
			return json.getAsJsonObject("data").getAsJsonObject("Typeahead_autocomplete").getAsJsonArray("results").get(3).getAsJsonObject().getAsJsonObject("detailsV2").getAsJsonObject("names").get("name").getAsString();
		}
		// If the value doesn't exist in any of the paths, return null or handle accordingly
		else {
			return "Parsing the JSON wasn't fruitful";
		}
	}

	public String getDest(String city) {
		String url = "https://travel-advisor.p.rapidapi.com/locations/v2/auto-complete?query="
		+ city + "&lang=en_US&units=km";
		// note to self: change API if/when possible due to inconsistent structure and lack of usefulness.
		try {
			HttpClient httpClient = HttpClient.newHttpClient();		    
			HttpRequest request = HttpRequest.newBuilder()
						.uri(URI.create(url))
						.header("X-RapidAPI-Key", "05f9700641msh334f29bfc3d23d9p1fa8bfjsn405a91632bc1")
						.header("X-RapidAPI-Host", "travel-advisor.p.rapidapi.com")
						.method("GET", HttpRequest.BodyPublishers.noBody())
						.build();

			HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());
			String responseBody = response.body();
			System.out.println(responseBody);
			JsonParser parser = new JsonParser();
			JsonObject json = parser.parse(responseBody).getAsJsonObject();
			String name = extractVal(json);
									
			return name;
		} catch (IOException | InterruptedException e) {
			e.printStackTrace();
			System.out.println("here1");
			return "Failed to fetch destination information for " + city;
		} catch (JsonParseException e) {
			e.printStackTrace();
			System.out.println("here2");
			return "Failed to fetch destination information for JsonIOException" + city;
		}
	}
	
	private JsonObject getTimeInfo(String city) throws IOException, InterruptedException {
		String API_KEY = "29332d7ba1294fc09e0e5e948623493c";
		String url = "https://timezone.abstractapi.com/v1/current_time/?api_key="+ API_KEY + "&location=" + city;

		try{
			HttpClient httpClient = HttpClient.newHttpClient();
			HttpRequest request = HttpRequest.newBuilder().uri(URI.create(url)).build();

			HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());
			String responseBody = response.body();
			System.out.println(responseBody);			

			JsonParser parser = new JsonParser();
			JsonObject json = parser.parse(responseBody).getAsJsonObject();
			return json;
		} catch	(IOException | InterruptedException e) {
			e.printStackTrace();
			Gson gson = new Gson();
			String err = "Failed to fetch time information for " + city;
			System.out.println(err);
			JsonObject jsonObject = gson.fromJson(err, JsonObject.class);
			return jsonObject;
		}
	}
			
	private JsonObject getWeatherInfo(String city) throws IOException, InterruptedException {
		String url = "https://weather-by-api-ninjas.p.rapidapi.com/v1/weather" + "?city=" + city;

		try{
			HttpClient httpClient = HttpClient.newHttpClient();
			HttpRequest request = HttpRequest.newBuilder()
						.uri(URI.create(url))
						.header("X-RapidAPI-Key", "972d126dc4msh5ef3c5e3bfbdb1fp19fd18jsn8a41f55c29a8")
						.header("X-RapidAPI-Host", "weather-by-api-ninjas.p.rapidapi.com")
						.method("GET", HttpRequest.BodyPublishers.noBody())
						.build();

			HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());
			String responseBody = response.body();
			System.out.println(responseBody);
			JsonParser parser = new JsonParser();
			JsonObject json = parser.parse(responseBody).getAsJsonObject();
			return json;
		} catch	(IOException | InterruptedException e) {
			e.printStackTrace();
			Gson gson = new Gson();
			String err = "Failed to fetch weather information for " + city;
			JsonObject jsonObject = gson.fromJson(err, JsonObject.class);
			System.out.println(err);
			return jsonObject;
		}
	}
		

	@GET
	@Produces(MediaType.APPLICATION_JSON)
	public BuddyMessage buddyService(@DefaultValue("") @QueryParam("msg1") String city1, @DefaultValue("") @QueryParam("msg2") String city2){
		String dest1 = getDest(city1);
		String dest2 = getDest(city2);				
		String info = "<p><strong>Places to Visit</strong></p>";
		info += "Destination to visit in " + city1 + ": " + dest1 + "<br>";
		info += "Destination to visit in " + city2 + ": " + dest2 + "<br><br> ";
		System.out.println("into method");
		
		try {	
			info += "<p><strong>DateTime Information</strong></p>";					
			
			JsonObject timeInfo1 = getTimeInfo(city1);
			JsonObject timeInfo2 = getTimeInfo(city2);			
            
			String datetime1 = timeInfo1.get("datetime").getAsString();
			String timezoneName1 = timeInfo1.get("timezone_name").getAsString();
			String timezoneAbbv1 = timeInfo1.get("timezone_abbreviation").getAsString();
			String gmtOffset1 = Integer.toString(timeInfo1.get("gmt_offset").getAsInt());
       			datetime1 += " GMT+" + gmtOffset1;       			
       			
       			String datetime2 = timeInfo2.get("datetime").getAsString();
			String timezoneName2 = timeInfo2.get("timezone_name").getAsString();
			String timezoneAbbv2 = timeInfo2.get("timezone_abbreviation").getAsString();
			String gmtOffset2 = Integer.toString(timeInfo2.get("gmt_offset").getAsInt());
       			datetime2 += " GMT+" + gmtOffset2;
       			
       		 	info += "Time in " + city1 + ": " + datetime1 + " " + timezoneName1 + " " + timezoneAbbv1 + "<br>";
			info += "Time in " + city2 + ": " + datetime2 + " " + timezoneName2 + " " + timezoneAbbv2 + "<br><br> ";
			
			info += "<p><strong>Weather Information</strong></p>";
			
			JsonObject weatherInfo1 = getWeatherInfo(city1);
			JsonObject weatherInfo2 = getWeatherInfo(city2);	
				
			double windSpeed1 = weatherInfo1.getAsJsonPrimitive("wind_speed").getAsDouble();
			double windSpeed2 = weatherInfo2.getAsJsonPrimitive("wind_speed").getAsDouble();
			
			info += "Wind Speed in " + city1 + ": " + windSpeed1 + "<br>";		        
			info += "Wind Speed in " + city2 + ": " + windSpeed2 + "<br><br>";
			
			int cloudPct1 = weatherInfo1.getAsJsonPrimitive("cloud_pct").getAsInt();
			int cloudPct2 = weatherInfo2.getAsJsonPrimitive("cloud_pct").getAsInt();
			
			info += "Cloud cover in " + city1 + ": " + cloudPct1 + "%<br>";	 		
			info += "Cloud cover in " + city2 + ": " + cloudPct2 + "%<br><br>";
			
			double minTemp1 = weatherInfo1.getAsJsonPrimitive("min_temp").getAsDouble();
			double minTemp2 = weatherInfo2.getAsJsonPrimitive("min_temp").getAsDouble();
			
			info += "Minimum Temperature in " + city1 + ": " + minTemp1 + " degrees<br>";	 		
			info += "Minimum Temperature in " + city2 + ": " + minTemp2 + " degrees<br><br>";
			
			double maxTemp1 = weatherInfo1.getAsJsonPrimitive("max_temp").getAsDouble();
			double maxTemp2 = weatherInfo2.getAsJsonPrimitive("max_temp").getAsDouble();
			
			info += "Maximum Temperature in " + city1 + ": " + maxTemp1 + " degrees<br>";
			info += "Maximum Temperature in " + city2 + ": " + maxTemp2 + " degrees<br><br>";
			
			int humidity1 = weatherInfo1.getAsJsonPrimitive("humidity").getAsInt();
			int humidity2 = weatherInfo2.getAsJsonPrimitive("humidity").getAsInt();

			info += "Humidity in " + city1 + ": " + humidity1 + " %<br>";
			info += "Humidity in " + city2 + ": " + humidity2 + " %<br><br>";

			
		} catch (Exception e) {
			e.printStackTrace();
			System.out.println("here");
			info += "Failed to fetch time or weather information.";
		}

		return new BuddyMessage(info);
	}
}
