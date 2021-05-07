<script>

    let messages = [];

    for (let i = 0; i < 20; i++) {
        let msg = "Coucou : " + i;
        messages = [...messages, msg];
    }

</script>


<div class="flex h-full overflow-auto space-x-3">

    <div class="w-1/3 bg-red-200">
        <!-- Column Content -->
        <div class="flex justify-center items-center">
            <div class="flex-1 relative">
                <div class="box border rounded flex flex-col shadow bg-white">
                    <div class="box__title bg-grey-lighter px-3 py-2 border-b"><h3
                            class="text-sm text-grey-darker font-medium">Messages</h3></div>
                    <!--                    <textarea placeholder="hey" class="text-grey-darkest flex-1 p-2 m-1 bg-transparent" name="tt">hello world</textarea>-->

                    <ul>
                        {#each messages as msg}
                            <li>
                                {msg}
                            </li>
                        {/each}
                    </ul>
                </div>
            </div>
        </div>
    </div>

    <div class="w-2/3  bg-blue-500">
        <!-- Column Content -->
        <div class="flex justify-center items-center">
            <div class="flex-1 relative ">
                <div class="box border rounded flex flex-col shadow bg-white ">
                    <div class="box__title bg-grey-lighter px-3 py-2 border-b">
                        <h3
                                class="text-sm text-grey-darker font-medium">Contenu</h3>
                    </div>
                    <!--                    <textarea placeholder="hey" class="text-grey-darkest flex-1 p-2 m-1 bg-transparent" name="tt">hello world</textarea>-->
                    <span>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent in nulla sit amet turpis facilisis dapibus. Praesent vulputate elementum elit ac dictum. Sed placerat orci lorem, nec pellentesque massa hendrerit eu. Nam cursus pretium convallis. Duis a sapien pellentesque orci rutrum ullamcorper. Nullam sagittis interdum ipsum, in gravida sapien egestas nec. Maecenas laoreet eget nisl vitae pulvinar. Fusce commodo massa maximus, placerat odio sit amet, cursus augue. Ut dignissim odio at ligula laoreet, quis euismod sem auctor. Cras ullamcorper purus id lorem consectetur, nec dictum diam convallis. Nam rutrum sapien vel erat volutpat, in porta orci sollicitudin. Vestibulum pretium mauris sapien. Quisque iaculis cursus lacinia. Donec lobortis justo eget sem tincidunt, commodo consectetur neque rutrum. Proin elementum efficitur nibh, id vehicula tortor volutpat vitae. Aliquam quis mauris vel ante accumsan tempus dapibus vitae massa.

Nam et turpis varius purus sagittis vestibulum. Nulla sagittis, erat eu hendrerit suscipit, ante ex viverra ipsum, sit amet lacinia ante nisl at est. Praesent purus augue, bibendum sit amet tortor quis, accumsan egestas ante. Nulla auctor felis non feugiat fringilla. Aenean laoreet nisi egestas purus rhoncus accumsan. Vivamus vitae libero in enim eleifend imperdiet. Phasellus commodo eros at placerat volutpat. Nullam rutrum leo dui, id vestibulum elit efficitur a.

Proin gravida nibh sit amet ultricies facilisis. Sed finibus ex eu mauris cursus, nec auctor velit pellentesque. Phasellus sed auctor sapien. Etiam efficitur magna nec dolor feugiat cursus. Mauris tristique, purus eget posuere vulputate, lacus odio tincidunt odio, in tincidunt nibh ex pulvinar urna. Sed pretium, sem sit amet congue ultrices, erat justo molestie mauris, et euismod est eros vitae nisl. Vivamus sollicitudin rutrum dignissim.

Proin imperdiet erat quis tortor placerat fermentum. Nunc luctus risus vel nunc gravida hendrerit. Nunc turpis risus, sollicitudin sed erat quis, sodales malesuada tortor. Proin consectetur hendrerit ante ac condimentum. Nulla diam sapien, imperdiet id interdum viverra, aliquam ut diam. Phasellus a sapien ligula. Pellentesque aliquam dignissim efficitur. Vivamus auctor nulla pellentesque lacus volutpat, non blandit est consequat. Proin vitae diam eu lacus consequat auctor. Vestibulum purus dui, pretium feugiat nisl id, maximus fermentum odio. Nulla viverra magna a porta imperdiet. Ut dolor sapien, gravida ut interdum at, dictum sit amet leo. Duis condimentum dui a eros molestie placerat. Integer vel risus vel sapien ornare gravida. Nulla non feugiat elit.

Maecenas et convallis magna. Integer mi nisl, vehicula ut sem quis, euismod maximus dui. Sed aliquet non augue sed fringilla. Ut mattis feugiat diam, eget pulvinar sem tincidunt ac. Quisque dapibus placerat mauris eu pulvinar. Suspendisse nec urna eu urna volutpat scelerisque. Nulla sodales eros vel bibendum faucibus. Nam vestibulum condimentum nisi at lacinia. Integer a lacus nec ante tempor elementum.</span>
                </div>
            </div>
        </div>
    </div>
</div>

<style>

</style>
