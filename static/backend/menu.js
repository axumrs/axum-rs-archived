          $(function(){
              let url = location.pathname;
              let prefix = '/admin';
              $('.sidebar a.nav-link').each(function(){
                  let href = $(this).attr('href');
                  if (href === url) {
                      if (href !== prefix) {
                      $(this).parent().parent().parent().addClass('menu-open').addClass('active');
                      }
                      $(this).addClass('active');
                      return;
                  }
              });
          });
