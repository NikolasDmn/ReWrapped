use yew::prelude::*;

#[function_component]
pub fn UploadGuide() -> Html {
    html! {
                <div class="upload-guide-container text-xl">
                   <div class="header flex items-center">
        <a class="logo-container w-1/3 flex flex-col items-center mb-20" href="/">
            <img src="assets/logo/logo.svg" alt="logo" class="logo mb-4 w-60" />
            <h2 class="text-4xl text-center"> { "ReWrapped" } </h2>
        </a>
        <div class="title-container w-3/4 ">
            <h1 class="text-8xl text-center"> { "Upload Guide" } </h1>
        </div>
    </div>
                    <p>
                        { "This website is in no way affiliated with Spotify. Additionally, the accessible Spotify APIs do not provide sufficiently detailed playback records. Because of this, you'll need to manually download your user data from Spotify and submit it to this site." }
                    </p>

                    <h3 class="text-2xl font-semibold mb-2 mt-4"> { "How to Download Your User Data" } </h3>
                    <p>
                        { "Thankfully, Spotify makes it relatively straightforward to download your data. However, please note that it might take a few days for Spotify to gather the necessary data for you. To request your data, follow these steps:" }
                    </p>
                    <ul class="steps-list">
                        <li>
                            { "- Go to Spotify's " }
                            <a href="https://www.spotify.com/account/privacy/" class="text-primary"> { "Privacy page" } </a>
                        </li>
                        <li>
                            { "- Scroll down to the " }
                            <span class="text-primary"> { "\"Download your data\"" } </span>
                            { " section." }
                        </li>
                        <li>
                            { "- Check the " }
                            <span class="text-primary"> { "\"Select Extended Streaming History\"" } </span>
                            { " option." }
                        </li>
                        <li>
                            { "- Click on " }
                            <span class="text-primary"> { "\"Request Data\"" } </span>
                            { "." }
                        </li>
                    </ul>
                    <p>
                        { "You should receive an email confirming your request. Once Spotify has processed your request, you'll receive another email with a download link to your data." }
                    </p>

                    <h3 class="text-2xl font-semibold mb-2 mt-4"> { "How to Upload Your User Data" } </h3>
                    <p>
                        { "After downloading your data from Spotify, unzip the files and upload only those in the following format: " }
                        <span class="text-primary"> { "\"Streaming_History_Audio_xxxx_yyyy_z.json\"" } </span>
                        { "." }
                    </p>

                    <h3 class="text-2xl font-semibold mb-2 mt-4"> { "An Important Note About Privacy" } </h3>
                    <p>
                        { "The files downloaded from Spotify are extremely sensitive. These files contain information such as any IP addresses you've used to listen to music, your country information, and a detailed breakdown of all your playback records (down to the millisecond)." }
                    </p>
                    <p>
                        { "This means you should take extra precautions when handling these files. To emphasize, you should handle this data with a great deal of caution: " }
                        <span class="text-primary"> { "\"A LOT OF PRECAUTIONS\"" } </span>
                        { "." }
                    </p>

                    <h3 class="text-2xl font-semibold mb-2 mt-4"> { "How This Website Handles Privacy" } </h3>
                    <p>
                        { "You might be concerned about handling such sensitive files—totally understandable! Let us clarify how this site ensures your privacy:" }
                    </p>
                    <ul class="privacy-concerns">
                        <li> { "- Your data does not leave your computer. EVER." } </li>
                        <li> { "- Everything runs locally in your browser through WebAssembly (Wasm)." } </li>
                        <li> { "- This ensures that no part of your data interacts with external servers." } </li>
                        <li> { "- Essentially, all processing happens client-side in your local environment for absolute privacy." } </li>
                        <li> { "- Even in your device nothing is stored in the browser (to avoid any malicious actor from accessing the browser storage). This is also the reason why reloading makes the app completely reload: Everything is not persistent and a copy exists locally only while the browser window is open." } </li>
                    </ul>

                </div>
            }
}
