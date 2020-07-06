<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update a user</name>
   <tag></tag>
   <elementGuidId>bb45cba9-a57b-4a4d-a8f3-2a5691484780</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;user_id&quot;,
      &quot;value&quot;: &quot;${user_id}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;last_name&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;contact_no&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;merchant_ids&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJraWQiOiJNVGhpTUdRMVlXTXRaVE5pTkMwME9XVXlMVGc0TmpVdE5HTm1ZVFkwWkRrNVpHVmoiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiJkbDgyNDJpNWdmbzdxM2FsOGIyOGpiNDVqIiwic3ViIjoiY2I3ODQxMDQtYTFlNy00Yzk2LTkwMmMtODI5N2RlZWMwN2IwIiwic2NvcGUiOiJlbWFpbCBvcGVuaWQgcHJvZmlsZSIsImtpZCI6Ik1UaGlNR1ExWVdNdFpUTmlOQzAwT1dVeUxUZzROalV0TkdObVlUWTBaRGs1WkdWaiIsImlzcyI6Imh0dHBzOlwvXC91c2Vycy52aXNlbnplLmNvbSIsImdyb3VwcyI6WyJtZXJjaGFudF9hZG1pbiJdLCJleHAiOjE1OTM1Mjk3NzIsImlhdCI6MTU5MzQ4NjU3Miwibm9uY2UiOiJDVE9OcFk0bGZrbVZwTTVYVUk4MTdPSEMyeGV6dGh0SzM5MExaV25DIiwianRpIjoiNzRhZDYwYmMtNTU3ZS00YjBmLWI0YmMtMWU2YmQwNjczZWI4In0.NhDg9DMKPMR9elM9CmT1VV-dnm7HlM10bny2reYanzMEvffG8G6qwWP8q34TAVaU2UN8An2fF6epCE94LZVPZmu0H0nGvs0_R8LF883hnTuHL23PRkHN8xpRHpa_zoz1gwerFzkeaHjDQsqrBgxX061DDYSfVbg3nQRBAwfVIqORGwCvfdvv5r2AuhpMF8-hyKrRgV7u90LeztDh6kwftYfqn9BjL3yDv79hq-yI_Qs4-r3eDV0VPF-XU1w1X2ODrurKXCF64DaTJ1EO42c6MFA-bbypsvWA2vrIyTagtCPPtSRWkeLBfSTqGMRdTMPOhDY_Xyo9Nw6ZRkcI3qzR5Q</value>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/users/${user_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>7d209433-f2f1-4b2f-999f-c2b0a28c6ded</id>
      <masked>false</masked>
      <name>user_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
