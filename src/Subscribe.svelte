
<!--
<div class="h-100 w-full flex items-center justify-center bg-teal-lightest font-sans">
    <div class="bg-white rounded shadow p-6 m-4 w-full lg:w-3/4 lg:max-w-lg">
        <div class="mb-4">
            <h1 class="text-grey-darkest">Todo List</h1>
            <div class="flex mt-4">
                <input class="shadow appearance-none border rounded w-full py-2 px-3 mr-4 text-grey-darker" placeholder="Add Todo">
                <button class="flex-no-shrink p-2 border-2 rounded text-teal border-teal hover:text-white hover:bg-teal">Add</button>
            </div>
        </div>
        <div>
            <div class="flex mb-4 items-center">
                <p class="w-full text-grey-darkest">Add another component to Tailwind Components</p>
                <button class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green">Done</button>
                <button class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">Remove</button>
            </div>
            <div class="flex mb-4 items-center">
                <p class="w-full line-through text-green">Submit Todo App Component to Tailwind Components</p>
                <button class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-grey border-grey hover:bg-grey">Not Done</button>
                <button class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">Remove</button>
            </div>
        </div>
    </div>
</div>


<div class="rounded-lg overflow-hidden shadow-lg bg-white min-h-64 lg:w-1/4 sm:w-1/2 md:w-1/3">
    <p class="px-2 text-gray-600 mb-2 text-2xl font-thin px-4 pt-3">Contacts</p>
    <div class="px-2">
        <svg class="absolute z-50 m-1 text-blue-400" width="24" height="24" viewBox="0 0 24 24" fill="currentColor"
             xmlns="http://www.w3.org/2000/svg">
            <path
                    d="M14.71 14H15.5L20.49 19L19 20.49L14 15.5V14.71L13.73 14.43C12.59 15.41 11.11 16 9.5 16C5.90997 16 3 13.09 3 9.5C3 5.90997 5.90997 3 9.5 3C13.09 3 16 5.90997 16 9.5C16 11.11 15.41 12.59 14.43 13.73L14.71 14ZM5 9.5C5 11.99 7.01001 14 9.5 14C11.99 14 14 11.99 14 9.5C14 7.01001 11.99 5 9.5 5C7.01001 5 5 7.01001 5 9.5Z"
                    fill="black" fill-opacity="0.54" />
        </svg>
        <input type="text" class="pl-8 p-1 bg-gray-200 w-full rounded relative" placeholder="Search teams or members">
    </div>
    <div class="py-5 px-3">
        <div class="flex justify-between px-2 py-2">
            <p class="flex text-gray-700">
                <svg class="w-2 text-gray-500 mx-2" viewBox="0 0 8 8" fill="currentColor">
                    <circle cx="4" cy="4" r="3" />
                </svg>
                Tighten Co.
            </p>
            <p class="text-gray-500 font-thin">Team</p>
        </div>
        <div class="flex justify-between px-2 py-2 bg-blue-100 rounded">
            <p class="flex text-gray-700">
                <svg class="h2 w-2 text-teal-500 mx-2" viewBox="0 0 8 8" fill="currentColor">
                    <circle cx="4" cy="4" r="3" />
                </svg>
                Taylor Otwell
            </p>
            <p class="text-gray-500 font-thin">Member</p>
        </div>
        <div class="flex justify-between px-2 py-2">
            <p class="flex text-gray-700">
                <svg class="h2 w-2 text-gray-500 mx-2" viewBox="0 0 8 8" fill="currentColor">
                    <circle cx="4" cy="4" r="3" />
                </svg>
                Adam Wathan
            </p>
            <p class="text-gray-500 font-thin">Member</p>
        </div>
        <div class="flex justify-between px-2 py-2">
            <p class="flex text-gray-700">
                <svg class="h2 w-2 text-gray-500 mx-2" viewBox="0 0 8 8" fill="currentColor">
                    <circle cx="4" cy="4" r="3" />
                </svg>
                Ahmed Ashraf
            </p>
            <p class="text-gray-500 font-thin">Member</p>
        </div>
        <div class="flex justify-between px-2 py-2">
            <p class="flex text-gray-700">
                <svg class="h2 w-2 text-teal-500 mx-2" viewBox="0 0 8 8" fill="currentColor">
                    <circle cx="4" cy="4" r="3" />
                </svg>
                Laracasts
            </p>
            <p class="text-gray-500 font-thin">Team</p>
        </div>
    </div>
    <div class="bg-gray-300 flex flex-row-reverse px-2 py-3">
        <button class="bg-blue-500 py-2 px-4 rounded text-white">Invite</button>
        <button class="py-2 px-4 rounded text-gray-600">Cancel</button>
    </div>


    <div class="w-full max-w-screen-xl mx-auto px-6">
        <div class="flex justify-center p-4 px-3 py-10">
            <div class="w-full max-w-md">
                <div class="bg-white shadow-md rounded-lg px-3 py-2 mb-4">
                    <div class="block text-gray-700 text-lg font-semibold py-2 px-2">
                        Item List
                    </div>
                    <div class="flex items-center bg-gray-200 rounded-md">
                        <div class="pl-2">
                            <svg class="fill-current text-gray-500 w-6 h-6" xmlns="http://www.w3.org/2000/svg"
                                 viewBox="0 0 24 24">
                                <path class="heroicon-ui"
                                      d="M16.32 14.9l5.39 5.4a1 1 0 0 1-1.42 1.4l-5.38-5.38a8 8 0 1 1 1.41-1.41zM10 16a6 6 0 1 0 0-12 6 6 0 0 0 0 12z" />
                            </svg>
                        </div>
                        <input
                                class="w-full rounded-md bg-gray-200 text-gray-700 leading-tight focus:outline-none py-2 px-2"
                                id="search" type="text" placeholder="Search teams or members">
                    </div>
                    <div class="py-3 text-sm">
                        <div class="flex justify-start cursor-pointer text-gray-700 hover:text-blue-400 hover:bg-blue-100 rounded-md px-2 py-2 my-2">
                            <span class="bg-gray-400 h-2 w-2 m-2 rounded-full"></span>
                            <div class="flex-grow font-medium px-2">Tighten Co.</div>
                            <div class="text-sm font-normal text-gray-500 tracking-wide">Team</div>
                        </div>
                        <div class="flex justify-start cursor-pointer text-gray-700 hover:text-blue-400 hover:bg-blue-100 rounded-md px-2 py-2 my-2">
                            <span class="bg-green-400 h-2 w-2 m-2 rounded-full"></span>
                            <div class="flex-grow font-medium px-2">Taylor Otwell</div>
                            <div class="text-sm font-normal text-gray-500 tracking-wide">Member</div>
                        </div>
                        <div class="flex justify-start cursor-pointer text-gray-700 hover:text-blue-400 hover:bg-blue-100 rounded-md px-2 py-2 my-2">
                            <span class="bg-gray-400 h-2 w-2 m-2 rounded-full"></span>
                            <div class="flex-grow font-medium px-2">Adam Wathan</div>
                            <div class="text-sm font-normal text-gray-500 tracking-wide">Member</div>
                        </div>
                        <div class="flex justify-start cursor-pointer text-gray-700 hover:text-blue-400 hover:bg-blue-100 rounded-md px-2 py-2 my-2">
                            <span class="bg-gray-400 h-2 w-2 m-2 rounded-full"></span>
                            <div class="flex-grow font-medium px-2">Duke Street Studio Inc.</div>
                            <div class="text-sm font-normal text-gray-500 tracking-wide">Team</div>
                        </div>
                        <div class="flex justify-start cursor-pointer text-gray-700 hover:text-blue-400 hover:bg-blue-100 rounded-md px-2 py-2 my-2">
                            <span class="bg-green-400 h-2 w-2 m-2 rounded-full"></span>
                            <div class="flex-grow font-medium px-2">Jeffrey Wey</div>
                            <div class="text-sm font-normal text-gray-500 tracking-wide">Member</div>
                        </div>
                    </div>
                    <div class="block bg-gray-200 text-sm text-right py-2 px-3 -mx-3 -mb-2 rounded-b-lg">
                        <button class="hover:text-gray-600 text-gray-500 font-bold py-2 px-4">
                            Cancel
                        </button>
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            Invite
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>-->

<script defer src="https://use.fontawesome.com/releases/v5.14.0/js/all.js"></script>


<section class="h-screen w-screen bg-gray-200 flex flex-col-reverse sm:flex-row min-h-0 min-w-0 overflow-hidden">

    <main class="sm:h-full flex-1 flex flex-col min-h-0 min-w-0 overflow-auto">
      <!--  <nav class="border-b bg-white px-6 py-2 flex items-center min-w-0 h-14">
            <h1 class="font-semibold text-lg"></h1>
            <span class="flex-1"></span>
            <span class="mr-2">
        <input type="text" placeholder="Search"
               class="w-full border-2 px-2 py-1 border border-gray-300 rounded-sm focus:outline-none focus:ring-2 focus:ring-blue-600 focus:border-transparent bg-gray-300 focus:bg-gray-100" />
      </span>
            <button
                    class="ml-auto border rounded-full ml-2 w-10 h-10 text-center leading-none text-gray-200 bg-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:border-transparent">
                <i class="fas fa-user fill-current"></i>
            </button>
        </nav>-->
        <section class="flex-1 pt-3 md:p-6 lg:mb-0 lg:min-h-0 lg:min-w-0">
            <div class="flex flex-col lg:flex-row h-full w-full">

                <div class="border pb-2 lg:pb-0 w-full lg:max-w-sm px-3 flex flex-row lg:flex-col flex-wrap lg:flex-nowrap">

                    <!-- control content left -->
                    <div class="bg-red-200 w-full h-24 min-h-0 min-w-0 mb-4"></div>
                    <div class="bg-red-200 w-full h-24 min-h-0 min-w-0 mb-4"></div>

                </div>

                <div class="border h-full w-full lg:flex-1 px-3 min-h-0 min-w-0">

                    <!-- overflow content right -->
                    <div class="bg-green-200 w-full h-full min-h-0 min-w-0 overflow-auto">
                        <div class="bg-pink-600 w-screen h-64"></div>
                        <div class="bg-blue-600 w-full h-64"></div>
                        <div class="bg-purple-600 w-screen h-64"></div>
                        <div class="bg-red-600 w-full h-64"></div>
                        <div class="bg-yellow-600 w-screen h-64"></div>
                        <div class="bg-green-600 w-full h-64"></div>
                    </div>

                </div>

            </div>
        </section>
        <footer class="px-6 py-3 border-t flex w-full items-end">
            <p class="text-gray-600">Made by @codingsafari</p>
            <div class="flex-1"></div>
            <button
                    class="shadow-md ml-auto border rounded-full ml-2 w-14 h-14 text-center leading-none text-green-200 bg-green-600 hover:bg-green-500 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:border-transparent">
                <i class="fas fa-question fill-current"></i>
            </button>
        </footer>
    </main>
</section>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Nunito&display=swap");

    body {
        font-family: "Nunito", sans-serif;
    }

    main {
        font-size: clamp(0.9rem, 3vw, 1rem);
    }

    #page-icon img {
        -webkit-animation: cssfilter 3s infinite;
    }


    @-webkit-keyframes cssfilter {
        0%, 100%  {
            filter: invert(75%) drop-shadow(0px 0px 2px blue)
        }

        50% {
            filter: invert(0%) drop-shadow(0px 0px 1px teal);
        }
    }
</style>
