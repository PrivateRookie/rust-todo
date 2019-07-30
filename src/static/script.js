var app = angular.module("app", []);

app.directive("todoItem", function () {
  return {
    templateUrl: "static/todo-item.html"
  }
});

app.controller("ctrl", function ($scope, $http) {
  var basePath = "/api/events";
  var detailMap = {};

  $scope.getEvents = function () {
    return $http.get(basePath).then(function (reponse) {
      $scope.events = reponse.data;
    });
  };

  $scope.createEvent = function () {
    var event = {
      content: $scope.content,
      finished: false
    };

    $http.post(basePath, event).then(function () {
      $scope.getEvents();
      $scope.content = "";
    });
  };

  $scope.deleteEvent = function (id) {
    $http.delete(basePath + "/" + id).then(function () {
      refresh()
    });
  };

  $scope.updateStatus = function (item) {
    var body = {
      id: item.id,
      finished: !item.finished
    };
    $http.put(basePath, body).then(function () {
      refresh();
    });
  };

  $scope.toggleEvent = function (id) {
    if (angular.isUndefined(detailMap[id])) {
      detailMap[id] = false;
    }
    detailMap[id] = !detailMap[id];
  }

  $scope.showDetail = function (id) {
    return detailMap[id];
  }

  function refresh() {
    $scope.getEvents();
    $scope.detailMap = {};
  }

  refresh();
});