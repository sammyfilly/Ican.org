# Ican.org
manage : ftpsupport@blockchainexplorer.co.site
   Dns domain on vercel/turbo

   
Release Phases
Last updated on February 6, 2023
4 min read
Managing DNS Records
Learn how to add, verify, and remove DNS records for your domains on Vercel with this guide.
Table of Contents

Adding DNS Records
Verifying DNS Records
Removing DNS Records
DNS Email Presets
Once a domain has been added and is using Vercel's nameservers, all of its DNS Records can be found by selecting the domain from the Domains page. From there, you can view, add, verify, and remove the records. You can also adjust the email presets, if necessary.

This document will guide you through each of these options.

To make sure DNS records are applied, and to allow you to manage them, your domain needs to use Vercel's nameservers. If you are using a third-party domain, you will be provided with the Vercel nameservers to copy and use with your registrar.
Adding DNS Records
Selecting your Domain

On the dashboard, click on the Domains tab. From the Domains tab, click on a domain of your choice to view its Advanced Settings page.

Add DNS Record

Once on the Advanced Settings page of your domain, select the Enable Vercel DNS button to fill out the DNS Record form. Once complete, click on the Add button.



DNS Records form to add a new DNS Record.

You can then create a new DNS record with the following data:

Name: The prefix or location of the record. For www.example.com, the name argument would be www.
Type: Types can be A, AAAA, ALIAS, CAA, CNAME, MX, SRV, or TXT.
Value: The value of the record.
TTL: Default is 60 seconds. For advanced users, this value can be customized.
More: Some records will require more data. MX records, for example, will request "priority".
Once a DNS record has been added, it can take up to 24 hours to the DNS records to fully update and any local caches to be cleared.
Verifying DNS Records
Once DNS records have been changed, you may wish to check that these have been set correctly. There are many third-party tools that do this, such as DNS Checker and DNS Map - these show the state of your DNS records in different regions of the world.

You can also use the terminal to check the DNS record for your domain, examples of doing this for the records above are found below:

terminal
 
$ dig A api.example.com +short
Verifying the A record set for a domain using the terminal.

terminal
 
$ dig MX example.com +short
Verifying the MX record set for a domain using the terminal.

Removing DNS Records
If you need to remove DNS records, you can do so once again by using the DNS UI. Select the elipses (⋯) to access the menu then select Delete DNS Zone. Follow the prompts to delete the record.

Default records can't be removed. However, new records can override them if required.

Removing a DNS record from the DNS UI.
Removing a DNS record from the DNS UI.
Removing a DNS record from the DNS UI.
DNS Email Presets
Vercel does not provide an email service. To be able to receive emails through a Domain that you've added to Vercel, you need to add the respective DNS Records of type MX.

For most common third-party email providers, Vercel helps with this process by allowing you to add those missing DNS Records using so-called DNS Email Presets on your dashboard.

In order to make use of them, navigate to the Domains page of your Personal Account or Team, click the Domain of your choice and click the Add Email Preset button on the right:



Adding a DNS Email Preset by clicking the Add Email Preset button.

You will be presented with a list of commonly used third-party email providers. If yours is in the list, select it and the necessary MX DNS Records will automatically be configured on your Domain.

If your email provider is not in the list, please refer to their documentation and dashboard to find out which MX DNS Records you need to add.

Was this helpful?



supported.
Send
On this page

Adding DNS Records
Verifying DNS Records
Removing DNS Records
DNS Email Presets


Product
Infrastructure
Previews
Edge Functions
Analytics
Next.js
Turbo
Enterprise
CLI & API
Changelog

Explore
Docs
Pricing
Customers
Integrations
Templates
Resources
Guides
Help links mail: <onedrive@notificationmail.microsoft.com>
⌘K

Company
About
Blog
Careers
Contact Us
Next.js Conf
Open Source
Partners
Security
Privacy Policy

Legal
© 2023 Vercel Inc.



-------------------------------




Release Phases

Last updated on August  6, 2023
4 min read
author ✍️ : Samson Unyinmadu 
mail malito : <onedrive@notificationmail.microsoft.com>
Managing DNS Records : assign 
Learn how to add, verify, and remove DNS records for your domains on Vercel with this guide.
Table of Contents
* Adding DNS Records
* Verifying DNS Records
* Removing DNS Records
* DNS Email Presets
Once a domain has been added and is using Vercel's nameservers, all of its DNS Records can be found by selecting the domain from the Domains page. From there, you can view, add, verify, and remove the records. You can also adjust the email presets, if necessary.
This document will guide you through each of these options.

new email: openworkspacesource@gmail.com

To make sure DNS records are applied, and to allow you to manage them, your domain needs to use Vercel's nameservers. If you are using a third-party domain, you will be provided with the Vercel nameservers to copy and use with your registrar.
Adding DNS Records
Selecting your Domain
On the dashboard, click on the Domains tab. From the Domains tab, click on a domain of your choice to view its Advanced Settings page.
Add DNS Record
Once on the Advanced Settings page of your domain, select the Enable Vercel DNS button to fill out the DNS Record form. Once complete, click on the Add button.


DNS Records form to add a new DNS Record.
You can then create a new DNS record with the following data:
* 		Name: The prefix or location of the record. For www.example.com, the name argument would be www.
* 		Type: Types can be A, AAAA, ALIAS, CAA, CNAME, MX, SRV, or TXT.
* 		Value: The value of the record.
* 		TTL: Default is 60 seconds. For advanced users, this value can be customized.
* 		More: Some records will require more data. MX records, for example, will request "priority".

Once a DNS record has been added, it can take up to 24 hours to the DNS records to fully update and any local caches to be cleared.
Verifying DNS Records
Once DNS records have been changed, you may wish to check that these have been set correctly. There are many third-party tools that do this, such as DNS Checker and DNS Map - these show the state of your DNS records in different regions of the world.
You can also use the terminal to check the DNS record for your domain, examples of doing this for the records above are found below:

terminal

$ dig A api.example.com +short
Verifying the A record set for a domain using the terminal.

terminal

$ dig MX example.com +short
Verifying the MX record set for a domain using the terminal.
Removing DNS Records
If you need to remove DNS records, you can do so once again by using the DNS UI. Select the elipses (⋯) to access the menu then select Delete DNS Zone. Follow the prompts to delete the record.
Default records can't be removed. However, new records can override them if required.


Removing a DNS record from the DNS UI.
DNS Email Presets
Vercel does not provide an email service. To be able to receive emails through a Domain that you've added to Vercel, you need to add the respective DNS Records of type MX.
For most common third-party email providers, Vercel helps with this process by allowing you to add those missing DNS Records using so-called DNS Email Presets on your dashboard.
In order to make use of them, navigate to the Domains page of your Personal Account or Team, click the Domain of your choice and click the Add Email Preset button on the right:


Adding a DNS Email Preset by clicking the Add Email Preset button.
You will be presented with a list of commonly used third-party email providers. If yours is in the list, select it and the necessary MX DNS Records will automatically be configured on your Domain.
If your email provider is not in the list, please refer to their documentation and dashboard to find out which MX DNS Records you need to add.

Was this helpful?
supported.
Send : one
* On this page 
* Adding DNS Records
* Verifying DNS Records
* Removing DNS Records
* DNS Email Presets
* DNS Email preseting
* onedrive@notificationmail.microsoft.com
* approved:<onedrive@notificationmail.microsoft.com>
* @vercelbot <link>below email: true <onedrive@notificationmail.microsoft.com>
* @DNS Email <link> below email: true
  <onedrive@notificationmail.microsoft.com>

Product
* Infrastructure
* Previews
* Edge Functions
* Analytics
* Next.js
* Turbo
* Enterprise
* CLI & API
* Changelog
Explore
* Docs
* Pricing
* Customers
* Integrations
* Templates
* Resources
* Experts link email 📧: <onedrive@notificationmail.microsoft.com>
* Guides
* Help
* ⌘K
Company
* About
* Blog
* Careers
* Contact Us
* Next.js Conf
* Open Source
* Partners
* Security
* Privacy Policy
* Legal
© 2023 Vercel Inc.
