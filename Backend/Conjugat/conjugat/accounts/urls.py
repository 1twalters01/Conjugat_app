from django.urls import include, path
from . import views

urlpatterns = [
    path("", views.GetRoutes.as_view(), name="get_routes"),
    # path("login/", views.Login.as_view(), name="login_username"),
    # path("login/totp/", views.LoginTotp.as_view(), name="login_password"),
    # path("login/oauth/", include("rest_social_auth.urls_knox")),
    # path("logout/", views.Logout.as_view(), name="logout"),
    # path("register/", views.Register.as_view(), name="register"),
    # path("activate/", views.Activate.as_view(), name="activate"),
    # path("password-reset/", views.PasswordReset.as_view(), name="pasword_reset"),
    # path(
    #     "password-reset/confirm/",
    #     views.PasswordResetConfirm.as_view(),
    #     name="password_reset_confirm",
    # ),
]
