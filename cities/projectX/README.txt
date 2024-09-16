==================================================================
Instructions to compile, test and run ProjectX TripBuddy Code
==================================================================

1) Make sure to install Maven 3.1 and Java 1.7, and update paths to point to mvn and java. Set Maven to use the IIITA proxy, if required. Set the JAVA_HOME and M2_HOME environment variables to point to your Java and Maven base folders. Make sure java, javac and mvn are in your execution path.
---------------------------

2) Compile and run TripBuddy REST Web Service on "host1", passing the hostname of your machine as param
cd cities/projectX
mvn compile
mvn exec:java -Dexec.mainClass="projectX.TripServiceLauncher" -Dexec.args="'host1' '8081'"

You should be able to visit the web service URL http://host1:8081/projectX/trip?msg1=London&msg2=Allahabad from a browser (or using wget or curl commands) acting as a REST client. The response should be a JSON message.
---------------------------
3) Once you have the REST web service running from #2, compile and run the Web Form Server on "host2", passing base URL location of REST service and public hostname of "host2" as params

cd cities/projectX
mvn compile
mvn exec:java -Dexec.mainClass="projectX.TripBuddyLauncher" -Dexec.args="'http://host1:8081/projectX/' 'host2' '8080'"

You should be able to visit the webform on host2 using a browser. Submitting the form will trigger the REST client present in the form implementation and call the REST web service on host1.
http://host2:8080/projectX/web
---------------------------
	
	
